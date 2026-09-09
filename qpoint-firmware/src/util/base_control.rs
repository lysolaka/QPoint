use embassy_stm32::Peri;
use embassy_stm32::dac::{DacChannel, Value};
use embassy_stm32::gpio::{Level, Output, Pin, Speed};
use embassy_stm32::mode::Blocking;

use qpoint_common::measurement::BaseSource;

use crate::util;

/// Driver for the base terminal capable of sourcing and sinking current, and supplying voltage.
///
/// See [`BaseSource`] for capabilities.
pub struct BaseControl<'d> {
    sel1: Output<'d>,
    sel2: Output<'d>,
    dac: DacChannel<'d, Blocking>,
    circuit: BaseSource,
}

impl<'d> BaseControl<'d> {
    /// Construct the base terminal driver.
    pub fn new(
        sel1: Peri<'d, impl Pin>,
        sel2: Peri<'d, impl Pin>,
        mut dac: DacChannel<'d, Blocking>,
    ) -> Self {
        dac.set(Value::Bit12Right(0));

        Self {
            sel1: Output::new(sel1, Level::Low, Speed::Low),
            sel2: Output::new(sel2, Level::Low, Speed::Low),
            dac,
            circuit: BaseSource::HighZ,
        }
    }

    /// Select the circuit to drive the base terminal.
    pub fn select(&mut self, circuit: BaseSource) {
        defmt::trace!("Selecting: {:?}", circuit);
        // for the illusion of "safety"
        self.dac.set(Value::Bit12Right(0));

        self.circuit = circuit;
        let (sel2, sel1) = circuit.selection();
        self.sel1.set_level(sel1.into());
        self.sel2.set_level(sel2.into());
    }

    /// Set the DAC driving voltage in the units of LSB.
    ///
    /// The resolution is 12 bits. Values outside the permitted range are silently truncated.
    pub fn set_dac(&mut self, value: u16) {
        self.dac.set(Value::Bit12Right(value));
    }

    /// Set the DAC driving voltage to match the desired `value` on the output.
    ///
    /// The allowed ranges depend on the currently selected source:
    /// - [`BaseSource::HighZ`]: irrelevant (forced to 0).
    /// - [`BaseSource::ISource`]: `0..=150` uA,
    /// - [`BaseSource::ISink`]: `0..=150` uA,
    /// - [`BaseSource::VSource`]: `0..=5` V,
    ///
    /// The unit of `value` is:
    /// - uA for [`BaseSource::ISource`] and [`BaseSource::ISink`],
    /// - V for [`BaseSource::VSource`],
    /// - *ignored* for [`BaseSource::HighZ`] (`value` forced to 0).
    pub fn set_value(&mut self, value: f32) {
        let dac_value = match self.circuit {
            BaseSource::HighZ => 0,
            BaseSource::ISource | BaseSource::ISink => {
                self.circuit.dac_value(util::clamp(value, 0.0, 150.0))
            }
            BaseSource::VSource => self.circuit.dac_value(util::clamp(value, 0.0, 5.0)),
        };

        defmt::trace!("Setting {=f32} for {:?} (DAC value: {=u16})", value, self.circuit, dac_value);

        self.dac.set(Value::Bit12Right(dac_value));
    }
}

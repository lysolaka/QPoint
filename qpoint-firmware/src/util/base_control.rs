use embassy_stm32::Peri;
use embassy_stm32::dac::{DacChannel, Value};
use embassy_stm32::gpio::{Level, Output, Pin, Speed};
use embassy_stm32::mode::Blocking;

use crate::util;
use crate::util::DAC_1LSB;

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
        // for the illusion of "safety"
        self.set_dac(0);

        self.circuit = circuit;
        let (sel2, sel1) = circuit.selection();
        self.sel1.set_level(sel1);
        self.sel2.set_level(sel2);
    }

    /// Set the DAC driving voltage in the units of LSB.
    ///
    /// The resolution is 12 bits. Values outside the permitted range are silently truncated.
    pub fn set_dac(&mut self, value: u16) {
        self.dac.set(Value::Bit12Right(value));
    }

    /// Source `i` microamperes (uA) of current from the base terminal.
    ///
    /// The circuit can supply from 0 uA to 150 uA.
    pub fn source_current(&mut self, i: f32) {
        self.select(BaseSource::ISource);

        let value = self.circuit.dac_value(util::clamp(i, 0.0, 150.0));
        self.set_dac(value);
    }

    /// Sink `i` microamperes (uA) of current into the base terminal.
    ///
    /// The circuit can sink from 0 uA to 150 uA.
    pub fn sink_current(&mut self, i: f32) {
        self.select(BaseSource::ISink);

        let value = self.circuit.dac_value(util::clamp(i, 0.0, 150.0));
        self.set_dac(value);
    }

    /// Supply `u` volts (V) at the base terminal through a 15k resistor.
    ///
    /// The circuit can supply from 0 V to 5 V.
    pub fn supply_voltage(&mut self, u: f32) {
        self.select(BaseSource::VSource);

        let value = self.circuit.dac_value(util::clamp(u, 0.0, 5.0));
        self.set_dac(value);
    }
}

/// Circuit used to drive the base terminal.
#[derive(defmt::Format, Clone, Copy, PartialEq, Eq)]
pub enum BaseSource {
    /// High impedance.
    ///
    /// The base terminal is effectively disconnected (floating).
    HighZ,
    /// Current source.
    ///
    /// Creates a positive voltage at the base terminal, such that the current flowing **out** of this
    /// circuit is proportional to the setting voltage. Current range: [0, 150] uA.
    ISource,
    /// Current sink.
    ///
    /// Creates a positive voltage at the base terminal, such that the current flowing **into** this
    /// circuit is proportional to the setting voltage. Current range: [0, 150] uA.
    ISink,
    /// Voltage source.
    ///
    /// Connects the base terminal to a variable voltage source ([0, 5] V) through a 15k resistor.
    VSource,
}

impl BaseSource {
    /// Selection pin settings to select the given source in the order: `SEL2, SEL1`.
    fn selection(&self) -> (Level, Level) {
        match self {
            BaseSource::HighZ => (Level::Low, Level::Low),
            BaseSource::ISource => (Level::Low, Level::High),
            BaseSource::ISink => (Level::High, Level::Low),
            BaseSource::VSource => (Level::High, Level::High),
        }
    }

    /// DAC setting to drive the base terminal with the given value.
    ///
    /// The unit of the value is:
    /// - uA for `ISource` and `ISink`, 
    /// - V for `VSource`,
    /// - *ignored* for `HighZ` (returns 0).
    // - ISource | ISink: I [A] = U [V] / 22k => U [V] = I [A] * 22k => U [mV] = I [uA] / 22
    // - VSource: Uo [V] = Ui [V] * 1,515152 => Ui [V] = Uo [V] / 1,515152
    fn dac_value(&self, value: f32) -> u16 {
        match self {
            BaseSource::HighZ => 0,
            BaseSource::ISource | BaseSource::ISink => {
                libm::roundf((value * 22.0) / DAC_1LSB) as u16
            },
            BaseSource::VSource => {
                libm::roundf(value / (1.515152 * DAC_1LSB) * 1000.0) as u16
            },
        }
    }
}

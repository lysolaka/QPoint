use embassy_stm32::Peri;
use embassy_stm32::dac::{DacChannel, Value};
use embassy_stm32::gpio::{Level, Output, Pin, Speed};
use embassy_stm32::mode::Blocking;

use crate::util;
use crate::util::U5V_1LSB;

/// Driver for the collector terminal capable of supplying voltage.
///
/// See [`CollectorSource`] for capabilities.
pub struct CollectorControl<'d> {
    sel1: Output<'d>,
    sel2: Output<'d>,
    dac: DacChannel<'d, Blocking>,
    circuit: CollectorSource,
}

impl<'d> CollectorControl<'d> {
    /// Construct the collector terminal driver.
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
            circuit: CollectorSource::HighZ,
        }
    }

    /// Select the circuit to drive the collector terminal.
    pub fn select(&mut self, circuit: CollectorSource) {
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

    /// Supply `u` volts (V) to the collector terminal.
    ///
    /// The circuit can supply from 0 V to 5 V.
    pub fn supply_voltage(&mut self, u: f32) {
        self.select(CollectorSource::VSource);

        let value = self.circuit.dac_value(util::clamp(u, 0.0, 5.0));
        self.set_dac(value);
    }
}

/// Circuit used to drive the collector terminal.
#[derive(defmt::Format, Clone, Copy, PartialEq, Eq)]
pub enum CollectorSource {
    /// High impedance.
    ///
    /// The collector terminal is effectively disconnected (floating).
    HighZ,
    /// +5 V.
    ///
    /// Connects the collector directly to the 5 V power supply.
    VCC,
    /// Voltage source.
    ///
    /// Connects the collector terminal to a variable voltage source ([0, 5] V).
    VSource,
    /// GND.
    ///
    /// Connects the collector directly to ground.
    GND,
}

impl CollectorSource {
    /// Selection pin settings to select the given source in the order: `SEL2, SEL1`.
    fn selection(&self) -> (Level, Level) {
        match self {
            CollectorSource::HighZ => (Level::Low, Level::Low),
            CollectorSource::VCC => (Level::Low, Level::High),
            CollectorSource::VSource => (Level::High, Level::Low),
            CollectorSource::GND => (Level::High, Level::High),
        }
    }

    /// DAC setting to drive the base terminal with the given value.
    ///
    /// The unit of the value is:
    /// - V for `VSource`,
    /// - *ignored* for `HighZ`, `VCC`, `GND` (returns 0).
    fn dac_value(&self, value: f32) -> u16 {
        match self {
            CollectorSource::VSource => libm::roundf((value * 1000.0) / U5V_1LSB) as u16,
            _ => 0,
        }
    }
}

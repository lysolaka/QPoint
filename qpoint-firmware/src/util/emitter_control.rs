use embassy_stm32::Peri;
use embassy_stm32::gpio::{Level, Output, Pin, Speed};

/// Driver for the emitter terminal.
///
/// See [`EmitterSource`] for capabilities.
pub struct EmitterControl<'d> {
    sel1: Output<'d>,
    sel2: Output<'d>,
    circuit: EmitterSource,
}

impl<'d> EmitterControl<'d> {
    /// Construct the emitter terminal driver.
    pub fn new(
        sel1: Peri<'d, impl Pin>,
        sel2: Peri<'d, impl Pin>,
    ) -> Self {
        Self {
            sel1: Output::new(sel1, Level::Low, Speed::Low),
            sel2: Output::new(sel2, Level::Low, Speed::Low),
            circuit: EmitterSource::HighZ,
        }
    }

    /// Select the circuit to drive the emitter terminal.
    pub fn select(&mut self, circuit: EmitterSource) {
        self.circuit = circuit;
        let (sel2, sel1) = circuit.selection();
        self.sel1.set_level(sel1);
        self.sel2.set_level(sel2);
    }
}

/// Circuit used to drive the emitter terminal.
#[derive(defmt::Format, Clone, Copy, PartialEq, Eq)]
pub enum EmitterSource {
    /// High impedance.
    ///
    /// The emitter terminal is effectively disconnected (floating).
    HighZ,
    /// +5 V.
    ///
    /// Connects the emitter directly to the 5 V power supply.
    VCC,
    /// GND.
    ///
    /// Connects the emitter directly to ground.
    GND,
}

impl EmitterSource {
    /// Selection pin settings to select the given source in the order: `SEL2, SEL1`.
    fn selection(&self) -> (Level, Level) {
        match self {
            EmitterSource::HighZ => (Level::Low, Level::Low),
            EmitterSource::VCC => (Level::Low, Level::High),
            EmitterSource::GND => (Level::High, Level::Low),
        }
    }
}

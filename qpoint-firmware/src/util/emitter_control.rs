use embassy_stm32::Peri;
use embassy_stm32::gpio::{Level, Output, Pin, Speed};

use qpoint_common::measurement::EmitterSource;

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
        defmt::trace!("Selecting: {:?}", circuit);

        self.circuit = circuit;
        let (sel2, sel1) = circuit.selection();
        self.sel1.set_level(sel1.into());
        self.sel2.set_level(sel2.into());
    }
}

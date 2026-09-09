//! Utility functions, structures and constants for use by the program.

mod adc_control;
mod base_control;
mod collector_control;
mod emitter_control;
pub mod sync;

pub use adc_control::AdcControl;
pub use base_control::BaseControl;
pub use collector_control::CollectorControl;
pub use emitter_control::EmitterControl;

use qpoint_common::Command;

/// Composite type for a [`Command`] to indicate its source.
#[derive(defmt::Format)]
pub struct MeasurementCommand {
    pub cmd: Command,
    pub source: CommandSource,
}

/// Place where the measurement command is coming from.
#[derive(defmt::Format)]
pub enum CommandSource {
    /// The command was sent by the on-board UI, the measurement result should be sent to its
    /// runner.
    UI,
    /// The command was sent by a remote application, the measurement result should be sent to the
    /// remote TX channel.
    Remote,
}

/// Clamp a value to an interval [`min`, `max`].
pub fn clamp<T: PartialOrd + Copy>(v: T, min: T, max: T) -> T {
    if v < min {
        min
    } else if v > max {
        max
    } else {
        v
    }
}

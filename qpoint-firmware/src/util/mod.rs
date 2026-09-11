//! Utility functions, structures and constants for use by the program.

pub mod color;
pub mod com;
pub mod measurement;
pub mod sync;

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

//! Channels and signals to be used by the program.

use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex as MutexT;
use embassy_sync::channel::Channel;

use super::MeasurementCommand;

/// Measurement commands queue.
pub static MEASUREMENT_CMD_Q: Channel<MutexT, MeasurementCommand, 4> = Channel::new();

//! Channels and signals to be used by the program.

use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex as MutexT;
use embassy_sync::channel::Channel;
use embassy_sync::mutex::Mutex;
use embassy_sync::signal::Signal;

use qpoint_common::Response;

use crate::util::MeasurementCommand;
use crate::util::display::DisplayUpdate;

/// Is a PC application attached.
pub static ATTACHED_M: Mutex<MutexT, bool> = Mutex::new(false);

/// Display updates queue.
pub static DISPLAY_UPDATE_Q: Channel<MutexT, DisplayUpdate, 4> = Channel::new();

/// Measurement commands queue.
pub static MEASUREMENT_CMD_Q: Channel<MutexT, MeasurementCommand, 4> = Channel::new();

/// Response transmit queue.
pub static RESPONSE_TX_Q: Channel<MutexT, Response, 4> = Channel::new();

/// RGB LED colour signal in the (R, G, B) format.
pub static RGB_LED_S: Signal<MutexT, (u8, u8, u8)> = Signal::new();

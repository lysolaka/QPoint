//! Utility functions, structures and constants for use by the program.

mod adc_control;
mod base_control;
mod collector_control;
mod emitter_control;

pub use adc_control::AdcControl;
pub use base_control::{BaseControl, BaseSource};
pub use collector_control::{CollectorControl, CollectorSource};
pub use emitter_control::{EmitterControl, EmitterSource};

/// 1 LSB for the ADC/DAC given in milivolts (mV) for mapping to 3,3V.
pub const U3V3_1LSB: f32 = 0.805664;
/// 1 LSB for the ADC/DAC given in milivolts (mV) for mapping to 5V.
pub const U5V_1LSB: f32 = 1.220703;

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

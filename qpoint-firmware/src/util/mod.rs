//! Utility functions, structures and constants for use by the program.

mod adc_control;
mod base_control;
mod collector_control;
mod emitter_control;

pub use adc_control::AdcControl;
pub use base_control::BaseControl;
pub use collector_control::CollectorControl;
pub use emitter_control::EmitterControl;

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

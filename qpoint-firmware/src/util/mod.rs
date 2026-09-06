//! Utility functions, structures and constants for use by the program.

mod base_control;
pub use base_control::{BaseControl, BaseSource};

/// 1 LSB for the DAC given in milivolts (mV).
pub const DAC_1LSB: f32 = 0.805664;

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

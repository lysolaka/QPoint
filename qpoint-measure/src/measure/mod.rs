// suppress warnings for now
// TODO: make code alive
#![allow(dead_code)]

use std::ops::RangeInclusive;

mod hparam;
mod input;
mod output;

pub use hparam::HybridParameters;
pub use input::InputCharacteristics;
pub use output::OutputCharacteristics;

/// Allowed voltage values (voltage source capability)
const VOLTAGE_RANGE: RangeInclusive<f32> = 0.0..=5.0;
/// Allowed current values (current source capability)
const CURRENT_RANGE: RangeInclusive<f32> = 0.0..=150.0;

fn parse_val(v: &str, name: &str) -> Result<f32, String> {
    v.parse()
        .map_err(|e| format!("Failed to parse {} value `{}`: {}", name, v, e))
}

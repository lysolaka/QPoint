// suppress warnings for now
// TODO: make code alive
#![allow(dead_code)]

use std::collections::HashMap;
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

#[derive(Clone, Debug)]
pub struct PlotData {
    title: String,
    data: HashMap<String, Vec<(f32, f32)>>,
}

impl PlotData {
    pub fn new(title: impl ToString) -> Self {
        Self {
            title: title.to_string(),
            data: HashMap::new(),
        }
    }

    pub fn push_series(&mut self, name: impl ToString, data: Vec<(f32, f32)>) {
        self.data.insert(name.to_string(), data);
    }
}

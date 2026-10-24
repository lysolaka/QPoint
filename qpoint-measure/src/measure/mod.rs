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

#[derive(Clone, Debug)]
pub struct PlotData {
    title: String,
    x_axis: Vec<f32>,
    y_axes: Vec<(String, Vec<f32>)>,
}

impl PlotData {
    pub fn new(title: impl Into<String>, x_axis: Vec<f32>, y_elements: usize) -> Self {
        Self {
            title: title.into(),
            x_axis,
            y_axes: Vec::with_capacity(y_elements),
        }
    }

    pub fn push_series(&mut self, name: impl Into<String>, data: Vec<f32>) {
        self.y_axes.push((name.into(), data));
    }
}

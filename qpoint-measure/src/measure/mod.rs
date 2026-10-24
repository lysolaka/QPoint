// suppress warnings for now
// TODO: make code alive
#![allow(dead_code)]

use std::iter;
use std::ops::RangeInclusive;

use kuva::prelude::*;

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
    x_label: String,
    y_label: String,
    x_axis: Vec<f32>,
    y_axes: Vec<(String, Vec<f32>)>,
}

impl PlotData {
    pub fn new(
        title: impl Into<String>,
        x_label: impl Into<String>,
        y_label: impl Into<String>,
        x_axis: Vec<f32>,
        y_elements: usize,
    ) -> Self {
        Self {
            title: title.into(),
            x_label: x_label.into(),
            y_label: y_label.into(),
            x_axis,
            y_axes: Vec::with_capacity(y_elements),
        }
    }

    pub fn push_series(&mut self, name: impl Into<String>, data: Vec<f32>) {
        self.y_axes.push((name.into(), data));
    }

    pub fn render_svg(&self) -> String {
        let plots: Vec<_> = self
            .y_axes
            .iter()
            .map(|(title, data)| {
                let points = iter::zip(self.x_axis.iter().copied(), data.iter().copied());

                LinePlot::new().with_data(points).with_legend(title)
            })
            .map(Plot::Line)
            .collect();

        let (x_min, x_max) = self
            .x_axis
            .iter()
            .copied()
            .fold((f32::INFINITY, f32::NEG_INFINITY), |(min, max), x| {
                (min.min(x), max.max(x))
            });

        let layout = Layout::auto_from_plots(&plots)
            .with_x_axis_min(x_min as f64)
            .with_x_axis_max(x_max as f64)
            .with_width(1280.0)
            .with_height(800.0)
            .with_scale(1.3)
            .with_title(&self.title)
            .with_x_label(&self.x_label)
            .with_y_label(&self.y_label)
            .with_minor_ticks(10)
            .with_show_grid(true)
            .with_show_minor_grid(true)
            .with_legend_position(LegendPosition::OutsideRightMiddle)
            .with_theme(Theme::light())
            .with_palette(Palette::bold());

        let mut svg = kuva::render_to_svg(plots, layout);
        svg.shrink_to_fit();
        svg
    }
}

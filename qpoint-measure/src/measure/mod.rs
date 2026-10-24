// suppress warnings for now
// TODO: make code alive
#![allow(dead_code)]

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
    series: Vec<PlotSeries>,
}

#[derive(Clone, Debug)]
pub struct PlotSeries {
    pub name: String,
    pub points: Vec<(f32, f32)>,
}

impl PlotData {
    pub fn new(
        title: impl Into<String>,
        x_label: impl Into<String>,
        y_label: impl Into<String>,
        num_series: usize,
    ) -> Self {
        Self {
            title: title.into(),
            x_label: x_label.into(),
            y_label: y_label.into(),
            series: Vec::with_capacity(num_series),
        }
    }

    pub fn push_series(&mut self, name: impl Into<String>, points: Vec<(f32, f32)>) {
        self.series.push(PlotSeries {
            name: name.into(),
            points,
        });
    }

    pub fn write_csv(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut wtr = csv::WriterBuilder::new()
            .delimiter(b';')
            .from_writer(Vec::new());

        wtr.write_record(["Series", &self.x_label, &self.y_label])?;

        for series in &self.series {
            for (x, y) in &series.points {
                wtr.write_record([series.name.as_str(), &x.to_string(), &y.to_string()])?;
            }
        }

        Ok(String::from_utf8(wtr.into_inner()?)?)
    }

    pub fn render_svg(&self) -> String {
        let plots: Vec<_> = self
            .series
            .iter()
            .map(|series| {
                let points = series.points.iter().copied();

                LinePlot::new().with_data(points).with_legend(&series.name)
            })
            .map(Plot::Line)
            .collect();

        let (x_min, x_max) = self
            .series
            .iter()
            .flat_map(|series| series.points.iter().map(|(x, _)| *x))
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

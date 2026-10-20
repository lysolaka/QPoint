// suppress warnings for now
// TODO: make code alive
#![allow(dead_code)]

mod hparam;
mod input;
mod output;

pub use hparam::HybridParameters;
pub use input::InputCharacteristics;
pub use output::OutputCharacteristics;

fn parse_val(v: &str, name: &str) -> Result<f32, String> {
    v.parse()
        .map_err(|e| format!("Failed to parse {} value `{}`: {}", name, v, e))
}

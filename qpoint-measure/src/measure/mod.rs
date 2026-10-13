// suppress warnings for now
// TODO: make code alive
#![allow(dead_code)]

pub mod hparam;
pub mod input;
pub mod output;

fn parse_val(v: &str, name: &str) -> Result<f32, String> {
    v.parse()
        .map_err(|e| format!("Failed to parse {} value `{}`: {}", name, v, e))
}

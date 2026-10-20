use crate::measure::parse_val;
use crate::measure::{CURRENT_RANGE, VOLTAGE_RANGE};

#[derive(Debug)]
pub struct OutputCharacteristics {
    base_currents: Vec<f32>,
    uce_start: f32,
    uce_end: f32,
    uce_step: f32,
}

impl OutputCharacteristics {
    pub fn new(
        conditions: &Vec<String>,
        start: &str,
        end: &str,
        step: &str,
    ) -> Result<Self, String> {
        if conditions.is_empty() {
            return Err("Failed to parse conditions: conditions must not be empty".to_string());
        }

        let mut base_currents: Vec<f32> = Vec::with_capacity(conditions.len());
        for cond in conditions.iter() {
            base_currents.push(parse_val(cond, "condition")?);
        }

        base_currents.sort_by(|a, b| a.total_cmp(b));

        let start = parse_val(start, "start")?;
        let end = parse_val(end, "start")?;
        let step = parse_val(step, "step")?;

        for ib in base_currents.iter() {
            if !CURRENT_RANGE.contains(ib) {
                return Err(format!("Base current: {} uA is out of range", ib));
            }
        }

        if !VOLTAGE_RANGE.contains(&start) {
            return Err(format!("Start value: {} V is out of range", start));
        }
        if !VOLTAGE_RANGE.contains(&end) {
            return Err(format!("End value: {} V is out of range", end));
        }
        if !VOLTAGE_RANGE.contains(&step) {
            return Err(format!("Step value: {} V is out of range", step));
        }

        Ok(Self {
            base_currents,
            uce_start: start,
            uce_end: end,
            uce_step: step,
        })
    }
}

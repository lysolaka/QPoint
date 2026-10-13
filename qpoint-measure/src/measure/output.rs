use crate::measure::parse_val;

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
            base_currents.push(
                parse_val(cond, "condition")?,
            );
        }

        base_currents.sort_by(|a, b| a.total_cmp(b));

        let start = parse_val(start, "start")?;
        let end = parse_val(end, "start")?;
        let step = parse_val(step, "step")?;

        // TODO: check ranges

        Ok(Self {
            base_currents,
            uce_start: start,
            uce_end: end,
            uce_step: step,
        })
    }
}

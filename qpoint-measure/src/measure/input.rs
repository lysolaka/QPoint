use crate::measure::parse_val;

#[derive(Debug)]
pub struct InputCharacteristics {
    ce_voltages: Vec<f32>,
    ub_start: f32,
    ub_end: f32,
    ub_step: f32,
}

impl InputCharacteristics {
    pub fn new(
        conditions: &Vec<String>,
        start: &str,
        end: &str,
        step: &str,
    ) -> Result<Self, String> {
        if conditions.is_empty() {
            return Err("Failed to parse conditions: conditions must not be empty".to_string());
        }

        let mut ce_voltages: Vec<f32> = Vec::with_capacity(conditions.len());
        for cond in conditions.iter() {
            ce_voltages.push(
                parse_val(cond, "condition")?,
            );
        }

        ce_voltages.sort_by(|a, b| a.total_cmp(b));

        let start = parse_val(start, "start")?;
        let end = parse_val(end, "start")?;
        let step = parse_val(step, "step")?;

        // TODO: check ranges

        Ok(Self {
            ce_voltages,
            ub_start: start,
            ub_end: end,
            ub_step: step,
        })
    }
}

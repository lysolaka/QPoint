use crate::measure::parse_val;

#[derive(Debug)]
pub struct HybridParameters {
    hie: InputImpedance,
    hre: ReverseTransfer,
    hfe: ForwardGain,
    hoe: OutputAdmittance,
}

impl HybridParameters {
    pub fn new(
        hie_start: &str,
        hie_end: &str,
        hie_step: &str,
        hre_current: &str,
        hre_voltage: &str,
        hfe_current1: &str,
        hfe_current2: &str,
        hfe_voltage: &str,
        hoe_current: &str,
        hoe_start: &str,
        hoe_end: &str,
        hoe_step: &str,
    ) -> Result<Self, String> {
        let hie_start = parse_val(hie_start, "UB Start")?;
        let hie_end = parse_val(hie_end, "UB End")?;
        let hie_step = parse_val(hie_step, "UB Step")?;

        let hre_current = parse_val(hre_current, "base current (reverse voltage transfer)")?;
        let hre_voltage = parse_val(
            hre_voltage,
            "collector-emitter voltage (reverse voltage transfer)",
        )?;

        let hfe_current1 = parse_val(hfe_current1, "base current 1")?;
        let hfe_current2 = parse_val(hfe_current2, "base current 2")?;
        let hfe_voltage = parse_val(
            hfe_voltage,
            "collector-emitter voltage (forward current gain)",
        )?;

        let hoe_current = parse_val(hoe_current, "base current (output admittance)")?;
        let hoe_start = parse_val(hoe_start, "UCE Start")?;
        let hoe_end = parse_val(hoe_end, "UCE End")?;
        let hoe_step = parse_val(hoe_step, "UCE Step")?;

        // TODO: check ranges

        Ok(Self {
            hie: InputImpedance {
                ub_start: hie_start,
                ub_end: hie_end,
                ub_step: hie_step,
            },
            hre: ReverseTransfer {
                base_current: hre_current,
                ce_voltage: hre_voltage,
            },
            hfe: ForwardGain {
                base_current1: hfe_current1,
                base_current2: hfe_current2,
                ce_voltage: hfe_voltage,
            },
            hoe: OutputAdmittance {
                base_current: hoe_current,
                uce_start: hoe_start,
                uce_end: hoe_end,
                uce_step: hoe_step,
            },
        })
    }
}

#[derive(Debug)]
struct InputImpedance {
    ub_start: f32,
    ub_end: f32,
    ub_step: f32,
}

#[derive(Debug)]
struct ReverseTransfer {
    base_current: f32,
    ce_voltage: f32,
}

#[derive(Debug)]
struct ForwardGain {
    base_current1: f32,
    base_current2: f32,
    ce_voltage: f32,
}

#[derive(Debug)]
struct OutputAdmittance {
    base_current: f32,
    uce_start: f32,
    uce_end: f32,
    uce_step: f32,
}

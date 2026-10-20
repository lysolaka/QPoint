use crate::measure::parse_val;
use crate::measure::{CURRENT_RANGE, VOLTAGE_RANGE};

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

        if !VOLTAGE_RANGE.contains(&hie_start) {
            return Err(format!("UB Start value: {} V is out of range", hie_start));
        }
        if !VOLTAGE_RANGE.contains(&hie_end) {
            return Err(format!("UB End value: {} V is out of range", hie_end));
        }
        if !VOLTAGE_RANGE.contains(&hie_step) {
            return Err(format!("UB Step value: {} V is out of range", hie_step));
        }

        if !CURRENT_RANGE.contains(&hre_current) {
            return Err(format!(
                "Reverse transfer base current: {} uA is out of range",
                hre_current
            ));
        }
        if !VOLTAGE_RANGE.contains(&hre_voltage) {
            return Err(format!(
                "Reverse transfer Collector-Emitter voltage: {} V is out of range",
                hre_voltage
            ));
        }

        if !CURRENT_RANGE.contains(&hfe_current1) {
            return Err(format!(
                "Forward gain base current (1): {} uA is out of range",
                hfe_current1
            ));
        }
        if !CURRENT_RANGE.contains(&hfe_current2) {
            return Err(format!(
                "Forward gain base current (2): {} uA is out of range",
                hfe_current2
            ));
        }
        if !VOLTAGE_RANGE.contains(&hfe_voltage) {
            return Err(format!(
                "Forward gain Collector-Emitter voltage: {} V is out of range",
                hfe_voltage
            ));
        }

        if !CURRENT_RANGE.contains(&hoe_current) {
            return Err(format!(
                "Output admittance base current: {} uA is out of range",
                hoe_current
            ));
        }
        if !VOLTAGE_RANGE.contains(&hoe_start) {
            return Err(format!("UCE Start value: {} V is out of range", hoe_start));
        }
        if !VOLTAGE_RANGE.contains(&hoe_end) {
            return Err(format!("UCE End value: {} V is out of range", hoe_end));
        }
        if !VOLTAGE_RANGE.contains(&hoe_step) {
            return Err(format!("UCE Step value: {} V is out of range", hoe_step));
        }

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

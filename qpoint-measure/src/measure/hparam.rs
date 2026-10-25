use iter_num_tools::arange;

use qpoint_common::measurement::{BaseSource, CollectorSource, EmitterSource};
use qpoint_common::{Command, Response};

use crate::com::{Device, Error};
use crate::measure::CURRENT_RANGE;
use crate::measure::VOLTAGE_RANGE;
use crate::measure::{HybridMatrix, parse_val};

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

    pub async fn measure_npn(self) -> Result<HybridMatrix, Error> {
        let mut dev = Device::connect()?;
        dev.lock().await?;

        let results = HybridMatrix {
            hie: self.hie.measure_npn(&mut dev).await?,
            hre: self.hre.measure_npn(&mut dev).await?,
            hfe: self.hfe.measure_npn(&mut dev).await?,
            hoe: self.hoe.measure_npn(&mut dev).await?,
        };

        dev.unlock().await?;

        Ok(results)
    }

    pub async fn measure_pnp(self) -> Result<HybridMatrix, Error> {
        let mut dev = Device::connect()?;
        dev.lock().await?;

        let results = HybridMatrix {
            hie: self.hie.measure_pnp(&mut dev).await?,
            hre: self.hre.measure_pnp(&mut dev).await?,
            hfe: self.hfe.measure_pnp(&mut dev).await?,
            hoe: self.hoe.measure_pnp(&mut dev).await?,
        };

        dev.unlock().await?;

        Ok(results)
    }
}

#[derive(Debug)]
struct InputImpedance {
    ub_start: f32,
    ub_end: f32,
    ub_step: f32,
}

impl InputImpedance {
    async fn measure_npn(&self, dev: &mut Device) -> Result<Option<f64>, Error> {
        let ub_sweep = arange(self.ub_start..(self.ub_end + self.ub_step), self.ub_step);

        dev.transaction(Command::EmitterSelect(EmitterSource::GND))
            .await?;
        dev.transaction(Command::CollectorSelect(CollectorSource::GND))
            .await?;
        dev.transaction(Command::BaseSelect(BaseSource::VSource))
            .await?;

        let mut data = Vec::with_capacity(ub_sweep.len());
        for ub in ub_sweep {
            let r = dev
                .transaction(Command::BaseSet {
                    value: ub,
                    measure: true,
                })
                .await?;
            match r {
                Response::Ack => return Err(Error::UnexpectedResponse),
                Response::Measurement(result) => {
                    let ib = ((ub * 1000.0) - result.base_voltage) / 15.0;
                    data.push((result.base_voltage, ib));
                }
            }
        }

        let lin_segment = LinearSegment::find(&data, 5, 0.1, 0.05);

        Ok(lin_segment.map(|v| v.slope))
    }

    async fn measure_pnp(&self, dev: &mut Device) -> Result<Option<f64>, Error> {
        let ub_sweep = arange(self.ub_start..(self.ub_end + self.ub_step), self.ub_step).rev();

        dev.transaction(Command::EmitterSelect(EmitterSource::VCC))
            .await?;
        dev.transaction(Command::CollectorSelect(CollectorSource::VCC))
            .await?;
        dev.transaction(Command::BaseSelect(BaseSource::VSource))
            .await?;

        let mut data = Vec::with_capacity(ub_sweep.len());
        for ub in ub_sweep.clone() {
            let r = dev
                .transaction(Command::BaseSet {
                    value: ub,
                    measure: true,
                })
                .await?;
            match r {
                Response::Ack => return Err(Error::UnexpectedResponse),
                Response::Measurement(result) => {
                    let ib = ((ub * 1000.0) - result.base_voltage) / 15.0;
                    data.push((result.base_voltage - 5000.0, ib));
                }
            }
        }

        let lin_segment = LinearSegment::find(&data, 5, 0.1, 0.05);

        Ok(lin_segment.map(|v| v.slope))
    }
}

#[derive(Debug)]
struct ReverseTransfer {
    base_current: f32,
    ce_voltage: f32,
}

impl ReverseTransfer {
    async fn measure_npn(&self, dev: &mut Device) -> Result<f32, Error> {
        dev.transaction(Command::EmitterSelect(EmitterSource::GND))
            .await?;
        dev.transaction(Command::CollectorSelect(CollectorSource::GND))
            .await?;
        dev.transaction(Command::BaseSelect(BaseSource::ISource))
            .await?;
        let r = dev
            .transaction(Command::BaseSet {
                value: self.base_current,
                measure: true,
            })
            .await?;
        let ube1 = match r {
            Response::Ack => return Err(Error::UnexpectedResponse),
            Response::Measurement(result) => result.base_voltage,
        };

        dev.transaction(Command::CollectorSelect(CollectorSource::VSource))
            .await?;
        let r = dev
            .transaction(Command::CollectorSet {
                value: self.ce_voltage,
                measure: true,
            })
            .await?;
        let ube2 = match r {
            Response::Ack => return Err(Error::UnexpectedResponse),
            Response::Measurement(result) => result.base_voltage,
        };

        Ok((ube2 - ube1) / (self.ce_voltage * 1000.0))
    }

    async fn measure_pnp(&self, dev: &mut Device) -> Result<f32, Error> {
        dev.transaction(Command::EmitterSelect(EmitterSource::VCC))
            .await?;
        dev.transaction(Command::CollectorSelect(CollectorSource::VCC))
            .await?;
        dev.transaction(Command::BaseSelect(BaseSource::ISink))
            .await?;
        let r = dev
            .transaction(Command::BaseSet {
                value: self.base_current,
                measure: true,
            })
            .await?;
        let ube1 = match r {
            Response::Ack => return Err(Error::UnexpectedResponse),
            Response::Measurement(result) => result.base_voltage - 5000.0,
        };

        dev.transaction(Command::CollectorSelect(CollectorSource::VSource))
            .await?;
        let r = dev
            .transaction(Command::CollectorSet {
                value: 5.0 - self.ce_voltage,
                measure: true,
            })
            .await?;
        let ube2 = match r {
            Response::Ack => return Err(Error::UnexpectedResponse),
            Response::Measurement(result) => result.base_voltage - 5000.0,
        };

        Ok((ube2 - ube1) / (self.ce_voltage * 1000.0))
    }
}

#[derive(Debug)]
struct ForwardGain {
    base_current1: f32,
    base_current2: f32,
    ce_voltage: f32,
}

impl ForwardGain {
    async fn measure_npn(&self, dev: &mut Device) -> Result<f32, Error> {
        dev.transaction(Command::EmitterSelect(EmitterSource::GND))
            .await?;
        dev.transaction(Command::CollectorSelect(CollectorSource::VSource))
            .await?;
        dev.transaction(Command::CollectorSet {
            value: self.ce_voltage,
            measure: false,
        })
        .await?;
        dev.transaction(Command::BaseSelect(BaseSource::ISource))
            .await?;
        let r = dev
            .transaction(Command::BaseSet {
                value: self.base_current1,
                measure: true,
            })
            .await?;
        let ic1 = match r {
            Response::Ack => return Err(Error::UnexpectedResponse),
            Response::Measurement(result) => result.collector_current,
        };

        let r = dev
            .transaction(Command::BaseSet {
                value: self.base_current2,
                measure: true,
            })
            .await?;
        let ic2 = match r {
            Response::Ack => return Err(Error::UnexpectedResponse),
            Response::Measurement(result) => result.collector_current,
        };

        Ok(1000.0 * (ic2 - ic1) / (self.base_current2 - self.base_current1))
    }

    async fn measure_pnp(&self, dev: &mut Device) -> Result<f32, Error> {
        dev.transaction(Command::EmitterSelect(EmitterSource::VCC))
            .await?;
        dev.transaction(Command::CollectorSelect(CollectorSource::VSource))
            .await?;
        dev.transaction(Command::CollectorSet {
            value: 5.0 - self.ce_voltage,
            measure: false,
        })
        .await?;
        dev.transaction(Command::BaseSelect(BaseSource::ISink))
            .await?;
        let r = dev
            .transaction(Command::BaseSet {
                value: self.base_current1,
                measure: true,
            })
            .await?;
        let ic1 = match r {
            Response::Ack => return Err(Error::UnexpectedResponse),
            Response::Measurement(result) => result.collector_current,
        };

        let r = dev
            .transaction(Command::BaseSet {
                value: self.base_current2,
                measure: true,
            })
            .await?;
        let ic2 = match r {
            Response::Ack => return Err(Error::UnexpectedResponse),
            Response::Measurement(result) => result.collector_current,
        };

        Ok(1000.0 * (ic2 - ic1) / (self.base_current2 - self.base_current1))
    }
}

#[derive(Debug)]
struct OutputAdmittance {
    base_current: f32,
    uce_start: f32,
    uce_end: f32,
    uce_step: f32,
}

impl OutputAdmittance {
    async fn measure_npn(&self, dev: &mut Device) -> Result<Option<f64>, Error> {
        let uce_sweep = arange(
            self.uce_start..(self.uce_end + self.uce_step),
            self.uce_step,
        );

        dev.transaction(Command::EmitterSelect(EmitterSource::GND))
            .await?;
        dev.transaction(Command::CollectorSelect(CollectorSource::VSource))
            .await?;
        dev.transaction(Command::BaseSelect(BaseSource::ISource))
            .await?;
        dev.transaction(Command::BaseSet {
            value: self.base_current,
            measure: false,
        })
        .await?;

        let mut data = Vec::with_capacity(uce_sweep.len());
        for uce in uce_sweep {
            let r = dev
                .transaction(Command::CollectorSet {
                    value: uce,
                    measure: true,
                })
                .await?;
            match r {
                Response::Ack => return Err(Error::UnexpectedResponse),
                Response::Measurement(result) => {
                    data.push((uce, result.collector_current));
                }
            }
        }

        let lin_segment = LinearSegment::find(&data, 5, 0.01, 0.05);

        Ok(lin_segment.map(|v| v.slope))
    }

    async fn measure_pnp(&self, dev: &mut Device) -> Result<Option<f64>, Error> {
        let uce_sweep = arange(
            self.uce_start..(self.uce_end + self.uce_step),
            self.uce_step,
        );

        dev.transaction(Command::EmitterSelect(EmitterSource::VCC))
            .await?;
        dev.transaction(Command::CollectorSelect(CollectorSource::VSource))
            .await?;
        dev.transaction(Command::CollectorSet {
            value: 5.0,
            measure: false,
        })
        .await?;
        dev.transaction(Command::BaseSelect(BaseSource::ISink))
            .await?;
        dev.transaction(Command::BaseSet {
            value: self.base_current,
            measure: false,
        })
        .await?;

        let mut data = Vec::with_capacity(uce_sweep.len());
        for uce in uce_sweep {
            let r = dev
                .transaction(Command::CollectorSet {
                    value: 5.0 - uce,
                    measure: true,
                })
                .await?;
            match r {
                Response::Ack => return Err(Error::UnexpectedResponse),
                Response::Measurement(result) => {
                    data.push((-uce, result.collector_current));
                }
            }
        }

        let lin_segment = LinearSegment::find(&data, 5, 0.01, 0.05);

        Ok(lin_segment.map(|v| v.slope))
    }
}

// TODO: this code will be alive once there's some logging
#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
struct LinearSegment {
    slope: f64,
    intercept: f64,
    normalized_residual: f64,
    start_index: usize,
    point_count: usize,
}

impl LinearSegment {
    fn find(
        points: &[(f32, f32)],
        min_points: usize,
        max_norm_residual: f64,
        y_scale_floor: f64,
    ) -> Option<Self> {
        let n = points.len();

        if min_points < 3
            || n < min_points
            || !max_norm_residual.is_finite()
            || max_norm_residual < 0.0
            || !y_scale_floor.is_finite()
            || y_scale_floor <= 0.0
        {
            return None;
        }

        if points
            .iter()
            .any(|&(x, y)| !x.is_finite() || !y.is_finite())
        {
            return None;
        }

        let mut best = None;

        for start in (0..=(n - min_points)).rev() {
            let suffix = &points[start..];
            let k = suffix.len() as f64;

            let mean_x = suffix.iter().map(|&(x, _)| x as f64).sum::<f64>() / k;
            let mean_y = suffix.iter().map(|&(_, y)| y as f64).sum::<f64>() / k;

            let mut sxx = 0.0;
            let mut sxy = 0.0;

            for &(x, y) in suffix {
                let dx = x as f64 - mean_x;
                let dy = y as f64 - mean_y;
                sxx += dx * dx;
                sxy += dx * dy;
            }

            if sxx <= 0.0 {
                continue;
            }

            let slope = sxy / sxx;
            let intercept = mean_y - slope * mean_x;

            let min_y = suffix
                .iter()
                .map(|p| p.1 as f64)
                .fold(f64::INFINITY, f64::min);

            let max_y = suffix
                .iter()
                .map(|p| p.1 as f64)
                .fold(f64::NEG_INFINITY, f64::max);

            let y_scale = mean_y.abs().max(max_y - min_y).max(y_scale_floor);

            let rss: f64 = suffix
                .iter()
                .map(|&(x, y)| {
                    let error = y as f64 - (intercept + slope * x as f64);
                    error * error
                })
                .sum();

            let norm_residual = (rss / k).sqrt() / y_scale;

            if norm_residual.is_finite() && norm_residual <= max_norm_residual {
                best = Some(LinearSegment {
                    slope,
                    intercept,
                    normalized_residual: norm_residual,
                    start_index: start,
                    point_count: suffix.len(),
                });
            }
        }

        best
    }
}

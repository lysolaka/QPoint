use iter_num_tools::arange;

use qpoint_common::measurement::{BaseSource, CollectorSource, EmitterSource};
use qpoint_common::{Command, Response};

use crate::com::{Device, Error};
use crate::measure::VOLTAGE_RANGE;
use crate::measure::{PlotData, parse_val};

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
            return Err("failed to parse conditions: conditions must not be empty".to_string());
        }

        let mut ce_voltages: Vec<f32> = Vec::with_capacity(conditions.len());
        for cond in conditions.iter() {
            ce_voltages.push(parse_val(cond, "condition")?);
        }

        ce_voltages.sort_by(|a, b| a.total_cmp(b));

        let start = parse_val(start, "start")?;
        let end = parse_val(end, "start")?;
        let step = parse_val(step, "step")?;

        for ib in ce_voltages.iter() {
            if !VOLTAGE_RANGE.contains(ib) {
                return Err(format!(
                    "collector-emitter voltage: {} V is out of range",
                    ib
                ));
            }
        }

        if !VOLTAGE_RANGE.contains(&start) {
            return Err(format!("start value: {} V is out of range", start));
        }
        if !VOLTAGE_RANGE.contains(&end) {
            return Err(format!("end value: {} V is out of range", end));
        }
        if !VOLTAGE_RANGE.contains(&step) {
            return Err(format!("step value: {} V is out of range", step));
        }

        Ok(Self {
            ce_voltages,
            ub_start: start,
            ub_end: end,
            ub_step: step,
        })
    }

    pub async fn measure_npn(self) -> Result<PlotData, Error> {
        log::info!(
            data:? = &self,
            note = "transistor type: NPN",
            note = "this may take some time",
            note = "this time depends on the number of conditions and voltages to step through";
            "starting input characteristics measurement"
        );
        let ub_sweep = arange(self.ub_start..(self.ub_end + self.ub_step), self.ub_step);

        let mut plot_data = PlotData::new(
            "Input Characteristics",
            "Base-Emitter Voltage [mV]",
            "Base Current [uA]",
            self.ce_voltages.len(),
        );

        let mut dev = Device::connect()?;
        dev.lock().await?;
        // setup initial conditions
        dev.transaction(Command::EmitterSelect(EmitterSource::GND))
            .await?;
        dev.transaction(Command::CollectorSelect(CollectorSource::VSource))
            .await?;
        dev.transaction(Command::BaseSelect(BaseSource::VSource))
            .await?;

        for cond in self.ce_voltages {
            log::debug!("stepping condition Uce = {} V", cond);
            dev.transaction(Command::CollectorSet {
                value: cond,
                measure: false,
            })
            .await?;
            let mut series = Vec::with_capacity(ub_sweep.len());

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
                        series.push((result.base_voltage, ib));
                    }
                }
            }

            plot_data.push_series(format!("Uce = {} V", cond), series);
        }

        dev.transaction(Command::BaseSelect(BaseSource::HighZ))
            .await?;
        dev.transaction(Command::CollectorSelect(CollectorSource::HighZ))
            .await?;
        dev.transaction(Command::EmitterSelect(EmitterSource::HighZ))
            .await?;

        dev.unlock().await?;
        Ok(plot_data)
    }

    pub async fn measure_pnp(self) -> Result<PlotData, Error> {
        log::info!(
            data:? = &self,
            note = "transistor type: PNP",
            note = "this may take some time",
            note = "this time depends on the number of conditions and voltages to step through";
            "starting input characteristics measurement"
        );
        let ub_sweep = arange(self.ub_start..(self.ub_end + self.ub_step), self.ub_step);

        let mut plot_data = PlotData::new(
            "Input Characteristics",
            "Base-Emitter Voltage [mV]",
            "Base Current [uA]",
            self.ce_voltages.len(),
        );

        let mut dev = Device::connect()?;
        dev.lock().await?;
        // setup initial conditions
        dev.transaction(Command::EmitterSelect(EmitterSource::VCC))
            .await?;
        dev.transaction(Command::CollectorSelect(CollectorSource::VSource))
            .await?;
        dev.transaction(Command::CollectorSet {
            value: 5.0,
            measure: false,
        })
        .await?;
        dev.transaction(Command::BaseSelect(BaseSource::VSource))
            .await?;
        dev.transaction(Command::BaseSet {
            value: 5.0,
            measure: false,
        })
        .await?;

        for cond in self.ce_voltages {
            log::debug!("stepping condition Uce = {} V", -cond);
            dev.transaction(Command::CollectorSet {
                value: 5.0 - cond,
                measure: false,
            })
            .await?;
            let mut series = Vec::with_capacity(ub_sweep.len());

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
                        series.push((result.base_voltage - 5000.0, ib));
                    }
                }
            }

            plot_data.push_series(format!("Uce = {} V", -cond), series);
        }

        dev.transaction(Command::BaseSelect(BaseSource::HighZ))
            .await?;
        dev.transaction(Command::CollectorSelect(CollectorSource::HighZ))
            .await?;
        dev.transaction(Command::EmitterSelect(EmitterSource::HighZ))
            .await?;

        dev.unlock().await?;
        Ok(plot_data)
    }
}

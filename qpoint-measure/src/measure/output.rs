use iter_num_tools::arange;

use qpoint_common::measurement::{BaseSource, CollectorSource, EmitterSource};
use qpoint_common::{Command, Response};

use crate::com::{Device, Error};
use crate::measure::CURRENT_RANGE;
use crate::measure::VOLTAGE_RANGE;
use crate::measure::{PlotData, parse_val};

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

    pub async fn measure_npn(self) -> Result<PlotData, Error> {
        let uce_sweep = arange(
            self.uce_start..(self.uce_end + self.uce_step),
            self.uce_step,
        );

        let mut plot_data = PlotData::new("Output Characteristics");

        let mut dev = Device::connect()?;
        dev.lock().await?;
        // setup initial conditions
        dev.transaction(Command::EmitterSelect(EmitterSource::GND))
            .await?;
        dev.transaction(Command::CollectorSelect(CollectorSource::VSource))
            .await?;
        dev.transaction(Command::BaseSelect(BaseSource::ISource))
            .await?;

        for cond in self.base_currents {
            dev.transaction(Command::BaseSet {
                value: cond,
                measure: false,
            })
            .await?;
            let mut series = Vec::with_capacity(uce_sweep.len());

            for uce in uce_sweep.clone() {
                let r = dev
                    .transaction(Command::CollectorSet {
                        value: uce,
                        measure: true,
                    })
                    .await?;
                match r {
                    Response::Ack => return Err(Error::UnexpectedResponse),
                    Response::Measurement(result) => {
                        series.push((uce, result.collector_current));
                    }
                }
            }

            plot_data.push_series(format!("Ib = {} uA", cond), series);
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
        let uce_sweep = arange(
            self.uce_start..(self.uce_end + self.uce_step),
            self.uce_step,
        )
        .rev();

        let mut plot_data = PlotData::new("Output Characteristics");

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
        dev.transaction(Command::BaseSelect(BaseSource::ISink))
            .await?;

        for cond in self.base_currents {
            dev.transaction(Command::BaseSet {
                value: cond,
                measure: false,
            })
            .await?;
            let mut series = Vec::with_capacity(uce_sweep.len());

            for uce in uce_sweep.clone() {
                let r = dev
                    .transaction(Command::CollectorSet {
                        value: uce,
                        measure: true,
                    })
                    .await?;
                match r {
                    Response::Ack => return Err(Error::UnexpectedResponse),
                    Response::Measurement(result) => {
                        series.push((uce - 5.0, result.collector_current));
                    }
                }
            }

            plot_data.push_series(format!("Ib = {} uA", -cond), series);
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

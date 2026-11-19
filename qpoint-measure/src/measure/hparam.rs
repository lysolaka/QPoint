use qpoint_common::measurement::{BaseSource, CollectorSource, EmitterSource};
use qpoint_common::{Command, Response};

use crate::com::{Device, Error};
use crate::measure::CURRENT_RANGE;
use crate::measure::VOLTAGE_RANGE;
use crate::measure::{HybridMatrix, parse_val};

#[derive(Debug)]
pub struct HybridParameters {
    u_ce: f32,
    i_b: f32,
    delta_u_ce: f32,
    delta_i_b: f32,
}

impl HybridParameters {
    pub fn new(u_ce: &str, i_b: &str, delta_u_ce: &str, delta_i_b: &str) -> Result<Self, String> {
        let u_ce = parse_val(u_ce, "collector-emitter voltage (operating point)")?;
        let i_b = parse_val(i_b, "base current (operating point)")?;

        let delta_u_ce = parse_val(delta_u_ce, "collector-emitter voltage (small-signal)")?;
        let delta_i_b = parse_val(delta_i_b, "base current (small-signal)")?;

        if !VOLTAGE_RANGE.contains(&u_ce) {
            return Err(format!(
                "collector-emitter voltage (operating point): {} V is out of range",
                u_ce
            ));
        }
        if !VOLTAGE_RANGE.contains(&(u_ce + delta_u_ce)) {
            return Err(format!(
                "collector-emitter voltage (small-signal, upper bound): {} V is out of range",
                u_ce + delta_u_ce
            ));
        }
        if !VOLTAGE_RANGE.contains(&(u_ce - delta_u_ce)) {
            return Err(format!(
                "collector-emitter voltage (small-signal, lower bound): {} V is out of range",
                u_ce - delta_u_ce
            ));
        }

        if !CURRENT_RANGE.contains(&i_b) {
            return Err(format!(
                "base current (operating point): {} uA is out of range",
                i_b
            ));
        }
        if !CURRENT_RANGE.contains(&(i_b + delta_i_b)) {
            return Err(format!(
                "base current (small-signal, upper bound): {} uA is out of range",
                i_b + delta_i_b
            ));
        }
        if !CURRENT_RANGE.contains(&(i_b - delta_i_b)) {
            return Err(format!(
                "base current (small-signal, lower bound): {} uA is out of range",
                i_b - delta_i_b
            ));
        }

        Ok(Self {
            u_ce,
            i_b,
            delta_u_ce,
            delta_i_b,
        })
    }

    pub async fn measure_npn(self) -> Result<HybridMatrix, Error> {
        log::info!(
            data:? = &self,
            note = "transistor type: NPN";
            "starting hybrid parameters measurement"
        );
        let mut dev = Device::connect()?;
        dev.lock().await?;

        let results = HybridMatrix {
            hie: self.measure_hie_npn(&mut dev).await?,
            hre: self.measure_hre_npn(&mut dev).await?,
            hfe: self.measure_hfe_npn(&mut dev).await?,
            hoe: self.measure_hoe_npn(&mut dev).await?,
        };

        dev.unlock().await?;

        Ok(results)
    }

    pub async fn measure_pnp(self) -> Result<HybridMatrix, Error> {
        log::info!(
            data:? = &self,
            note = "transistor type: PNP";
            "starting hybrid parameters measurement"
        );
        let mut dev = Device::connect()?;
        dev.lock().await?;

        let results = HybridMatrix {
            hie: self.measure_hie_pnp(&mut dev).await?,
            hre: self.measure_hre_pnp(&mut dev).await?,
            hfe: self.measure_hfe_pnp(&mut dev).await?,
            hoe: self.measure_hoe_pnp(&mut dev).await?,
        };

        dev.unlock().await?;

        Ok(results)
    }

    async fn measure_hie_npn(&self, dev: &mut Device) -> Result<f32, Error> {
        dev.transaction(Command::EmitterSelect(EmitterSource::GND))
            .await?;
        dev.transaction(Command::CollectorSelect(CollectorSource::VSource))
            .await?;
        dev.transaction(Command::BaseSelect(BaseSource::ISource))
            .await?;

        dev.transaction(Command::CollectorSet {
            value: self.u_ce,
            measure: false,
        })
        .await?;

        let u_be1 = match dev
            .transaction(Command::BaseSet {
                value: self.i_b - self.delta_i_b,
                measure: true,
            })
            .await?
        {
            Response::Ack => return Err(Error::UnexpectedResponse),
            Response::Measurement(r) => r.base_voltage,
        };

        let u_be2 = match dev
            .transaction(Command::BaseSet {
                value: self.i_b + self.delta_i_b,
                measure: true,
            })
            .await?
        {
            Response::Ack => return Err(Error::UnexpectedResponse),
            Response::Measurement(r) => r.base_voltage,
        };

        // [mV] / [uA] = [kOhm]
        Ok((u_be2 - u_be1) / (2.0 * self.delta_i_b))
    }

    async fn measure_hie_pnp(&self, dev: &mut Device) -> Result<f32, Error> {
        dev.transaction(Command::EmitterSelect(EmitterSource::VCC))
            .await?;
        dev.transaction(Command::CollectorSelect(CollectorSource::VSource))
            .await?;
        dev.transaction(Command::BaseSelect(BaseSource::ISink))
            .await?;

        dev.transaction(Command::CollectorSet {
            value: 5.0 - self.u_ce,
            measure: false,
        })
        .await?;

        let u_be1 = match dev
            .transaction(Command::BaseSet {
                value: self.i_b - self.delta_i_b,
                measure: true,
            })
            .await?
        {
            Response::Ack => return Err(Error::UnexpectedResponse),
            Response::Measurement(r) => r.base_voltage - 5000.0,
        };

        let u_be2 = match dev
            .transaction(Command::BaseSet {
                value: self.i_b + self.delta_i_b,
                measure: true,
            })
            .await?
        {
            Response::Ack => return Err(Error::UnexpectedResponse),
            Response::Measurement(r) => r.base_voltage - 5000.0,
        };

        // [mV] / [uA] = [kOhm]
        Ok(-(u_be2 - u_be1) / (2.0 * self.delta_i_b))
    }

    async fn measure_hre_npn(&self, dev: &mut Device) -> Result<f32, Error> {
        dev.transaction(Command::EmitterSelect(EmitterSource::GND))
            .await?;
        dev.transaction(Command::CollectorSelect(CollectorSource::VSource))
            .await?;
        dev.transaction(Command::BaseSelect(BaseSource::ISource))
            .await?;

        dev.transaction(Command::BaseSet {
            value: self.i_b,
            measure: false,
        })
        .await?;

        let u_be1 = match dev
            .transaction(Command::CollectorSet {
                value: self.u_ce - self.delta_u_ce,
                measure: true,
            })
            .await?
        {
            Response::Ack => return Err(Error::UnexpectedResponse),
            Response::Measurement(r) => r.base_voltage,
        };

        let u_be2 = match dev
            .transaction(Command::CollectorSet {
                value: self.u_ce + self.delta_u_ce,
                measure: true,
            })
            .await?
        {
            Response::Ack => return Err(Error::UnexpectedResponse),
            Response::Measurement(r) => r.base_voltage,
        };

        Ok((u_be2 - u_be1) / (1000.0 * 2.0 * self.delta_u_ce))
    }

    async fn measure_hre_pnp(&self, dev: &mut Device) -> Result<f32, Error> {
        dev.transaction(Command::EmitterSelect(EmitterSource::VCC))
            .await?;
        dev.transaction(Command::CollectorSelect(CollectorSource::VSource))
            .await?;
        dev.transaction(Command::BaseSelect(BaseSource::ISink))
            .await?;

        dev.transaction(Command::BaseSet {
            value: self.i_b,
            measure: false,
        })
        .await?;

        let u_be1 = match dev
            .transaction(Command::CollectorSet {
                value: 5.0 - (self.u_ce - self.delta_u_ce),
                measure: true,
            })
            .await?
        {
            Response::Ack => return Err(Error::UnexpectedResponse),
            Response::Measurement(r) => r.base_voltage - 5000.0,
        };

        let u_be2 = match dev
            .transaction(Command::CollectorSet {
                value: 5.0 - (self.u_ce + self.delta_u_ce),
                measure: true,
            })
            .await?
        {
            Response::Ack => return Err(Error::UnexpectedResponse),
            Response::Measurement(r) => r.base_voltage - 5000.0,
        };

        Ok((u_be2 - u_be1) / (1000.0 * 2.0 * self.delta_u_ce))
    }

    async fn measure_hfe_npn(&self, dev: &mut Device) -> Result<f32, Error> {
        dev.transaction(Command::EmitterSelect(EmitterSource::GND))
            .await?;
        dev.transaction(Command::CollectorSelect(CollectorSource::VSource))
            .await?;
        dev.transaction(Command::BaseSelect(BaseSource::ISource))
            .await?;

        dev.transaction(Command::CollectorSet {
            value: self.u_ce,
            measure: false,
        })
        .await?;

        let i_c1 = match dev
            .transaction(Command::BaseSet {
                value: self.i_b - self.delta_i_b,
                measure: true,
            })
            .await?
        {
            Response::Ack => return Err(Error::UnexpectedResponse),
            Response::Measurement(r) => r.collector_current,
        };

        let i_c2 = match dev
            .transaction(Command::BaseSet {
                value: self.i_b + self.delta_i_b,
                measure: true,
            })
            .await?
        {
            Response::Ack => return Err(Error::UnexpectedResponse),
            Response::Measurement(r) => r.collector_current,
        };

        Ok(1000.0 * (i_c2 - i_c1) / (2.0 * self.delta_i_b))
    }

    async fn measure_hfe_pnp(&self, dev: &mut Device) -> Result<f32, Error> {
        dev.transaction(Command::EmitterSelect(EmitterSource::VCC))
            .await?;
        dev.transaction(Command::CollectorSelect(CollectorSource::VSource))
            .await?;
        dev.transaction(Command::BaseSelect(BaseSource::ISink))
            .await?;

        dev.transaction(Command::CollectorSet {
            value: 5.0 - self.u_ce,
            measure: false,
        })
        .await?;

        let i_c1 = match dev
            .transaction(Command::BaseSet {
                value: self.i_b - self.delta_i_b,
                measure: true,
            })
            .await?
        {
            Response::Ack => return Err(Error::UnexpectedResponse),
            Response::Measurement(r) => r.collector_current,
        };

        let i_c2 = match dev
            .transaction(Command::BaseSet {
                value: self.i_b + self.delta_i_b,
                measure: true,
            })
            .await?
        {
            Response::Ack => return Err(Error::UnexpectedResponse),
            Response::Measurement(r) => r.collector_current,
        };

        Ok(1000.0 * (i_c2 - i_c1) / (2.0 * self.delta_i_b))
    }

    async fn measure_hoe_npn(&self, dev: &mut Device) -> Result<f32, Error> {
        dev.transaction(Command::EmitterSelect(EmitterSource::GND))
            .await?;
        dev.transaction(Command::CollectorSelect(CollectorSource::VSource))
            .await?;
        dev.transaction(Command::BaseSelect(BaseSource::ISource))
            .await?;

        dev.transaction(Command::BaseSet {
            value: self.i_b,
            measure: false,
        })
        .await?;

        let i_c1 = match dev
            .transaction(Command::CollectorSet {
                value: self.u_ce - self.delta_u_ce,
                measure: true,
            })
            .await?
        {
            Response::Ack => return Err(Error::UnexpectedResponse),
            Response::Measurement(r) => r.collector_current,
        };

        let i_c2 = match dev
            .transaction(Command::CollectorSet {
                value: self.u_ce + self.delta_u_ce,
                measure: true,
            })
            .await?
        {
            Response::Ack => return Err(Error::UnexpectedResponse),
            Response::Measurement(r) => r.collector_current,
        };

        // [mA] / [V] = [mS]
        Ok((i_c2 - i_c1) / (2.0 * self.delta_u_ce))
    }

    async fn measure_hoe_pnp(&self, dev: &mut Device) -> Result<f32, Error> {
        dev.transaction(Command::EmitterSelect(EmitterSource::VCC))
            .await?;
        dev.transaction(Command::CollectorSelect(CollectorSource::VSource))
            .await?;
        dev.transaction(Command::BaseSelect(BaseSource::ISink))
            .await?;

        dev.transaction(Command::BaseSet {
            value: self.i_b,
            measure: false,
        })
        .await?;

        let i_c1 = match dev
            .transaction(Command::CollectorSet {
                value: 5.0 - (self.u_ce - self.delta_u_ce),
                measure: true,
            })
            .await?
        {
            Response::Ack => return Err(Error::UnexpectedResponse),
            Response::Measurement(r) => r.collector_current,
        };

        let i_c2 = match dev
            .transaction(Command::CollectorSet {
                value: 5.0 - (self.u_ce + self.delta_u_ce),
                measure: true,
            })
            .await?
        {
            Response::Ack => return Err(Error::UnexpectedResponse),
            Response::Measurement(r) => r.collector_current,
        };

        // [mA] / [V] = [mS]
        Ok(-(i_c2 - i_c1) / (2.0 * self.delta_u_ce))
    }
}

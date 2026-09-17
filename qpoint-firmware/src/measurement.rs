//! Analog measurements runner.

use embassy_stm32::dac::Dac;
use embassy_time::Timer;

use qpoint_common::{Command, Response};

use crate::MeasurementResources;
use crate::util::CommandSource;
use crate::util::measurement::{AdcControl, BaseControl, CollectorControl, EmitterControl};
use crate::util::sync::MEASUREMENT_CMD_Q;
use crate::util::sync::RESPONSE_TX_Q;
use crate::util::sync::RESPONSE_UI_S;

#[embassy_executor::task]
pub async fn runner(r: MeasurementResources) -> ! {
    defmt::debug!("Configuring measurement runner...");

    let (collector_dac, base_dac) = Dac::new_blocking(r.dac, r.collector_dac, r.base_dac).split();
    let mut base_control = BaseControl::new(r.base_sel1, r.base_sel2, base_dac);
    let mut collector_control =
        CollectorControl::new(r.collector_sel1, r.collector_sel2, collector_dac);
    let mut emitter_control = EmitterControl::new(r.emitter_sel1, r.emitter_sel2);

    let mut adc_control = AdcControl::new(r.adc, r.dma, r.base_adc, r.collector_adc);

    defmt::info!("Measurement runner: OK!");

    loop {
        let command = MEASUREMENT_CMD_Q.receive().await;
        defmt::debug!("Running measurement command: {:?}", &command);

        let response = match command.cmd {
            Command::BaseSelect(source) => {
                base_control.select(source);
                Response::Ack
            }
            Command::BaseSet { value, measure } => {
                base_control.set_value(value);

                // give some time to settle
                Timer::after_millis(50).await;

                if measure {
                    // this `crate::Interrupts` looks so ugly, but it is what it is
                    let result = adc_control.measure(crate::Interrupts).await;
                    defmt::debug!("Measurement result: {:?}", &result);
                    Response::Measurement(result)
                } else {
                    Response::Ack
                }
            }
            Command::CollectorSelect(source) => {
                collector_control.select(source);
                Response::Ack
            }
            Command::CollectorSet { value, measure } => {
                collector_control.set_value(value);

                // give some time to settle
                Timer::after_millis(50).await;

                if measure {
                    // this `crate::Interrupts` looks so ugly, but it is what it is
                    let result = adc_control.measure(crate::Interrupts).await;
                    defmt::debug!("Measurement result: {:?}", &result);
                    Response::Measurement(result)
                } else {
                    Response::Ack
                }
            }
            Command::EmitterSelect(source) => {
                emitter_control.select(source);
                Response::Ack
            }
            _ => defmt::unreachable!(),
        };

        defmt::debug!("Sending back response: {:?}", &response);
        // TODO: send back the response to the UI
        match command.source {
            CommandSource::UI => {
                match response {
                    Response::Ack => (), // do nothing
                    Response::Measurement(result) => RESPONSE_UI_S.signal(result),
                }
            }
            CommandSource::Remote => RESPONSE_TX_Q.send(response).await,
        }
    }
}

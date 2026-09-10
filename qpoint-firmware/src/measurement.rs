//! Analog measurements runner.

use embassy_stm32::dac::Dac;

use qpoint_common::Command;

use crate::MeasurementResources;
use crate::util::CommandSource;
use crate::util::measurement::{AdcControl, BaseControl, CollectorControl, EmitterControl};
use crate::util::sync::MEASUREMENT_CMD_Q;

#[embassy_executor::task]
pub async fn runner(r: MeasurementResources) -> ! {
    defmt::debug!("Configuring measurement runner...");

    let (collector_dac, base_dac) = Dac::new_blocking(r.dac, r.collector_dac, r.base_dac).split();
    let mut base_control = BaseControl::new(r.base_sel1, r.base_sel2, base_dac);
    let mut collector_control =
        CollectorControl::new(r.collector_sel1, r.collector_sel2, collector_dac);
    let mut emitter_control = EmitterControl::new(r.emitter_sel1, r.emitter_sel2);

    let mut adc_control = AdcControl::new(r.adc, r.dma, r.base_adc, r.collector_adc);

    defmt::info!("Measurement runner: OK");

    loop {
        let command = MEASUREMENT_CMD_Q.receive().await;
        defmt::debug!("Running measurement command: {:?}", &command);
        match command.cmd {
            Command::BaseSelect(source) => base_control.select(source),
            Command::BaseSet { value, measure } => {
                base_control.set_value(value);

                if measure {
                    // this `crate::Interrupts` looks so ugly, but it is what it is
                    let result = adc_control.measure(crate::Interrupts).await;

                    defmt::debug!("Measurement done, sending back the result");
                    defmt::debug!("Measurement result: {:?}", &result);
                    // TODO: send back the result
                    match command.source {
                        CommandSource::UI => (),
                        CommandSource::Remote => (),
                    }
                }
            }
            Command::CollectorSelect(source) => collector_control.select(source),
            Command::CollectorSet { value, measure } => {
                collector_control.set_value(value);

                if measure {
                    // this `crate::Interrupts` looks so ugly, but it is what it is
                    let result = adc_control.measure(crate::Interrupts).await;

                    defmt::debug!("Measurement done, sending back the result");
                    defmt::debug!("Measurement result: {:?}", &result);
                    // TODO: send back the result
                    match command.source {
                        CommandSource::UI => (),
                        CommandSource::Remote => (),
                    }
                }
            }
            Command::EmitterSelect(source) => emitter_control.select(source),
            _ => defmt::panic!("Unexpected measurement command."),
        }
    }
}

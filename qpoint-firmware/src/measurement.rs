//! Analog measurements runner.

use embassy_stm32::dac::Dac;

use crate::MeasurementResources;
use crate::util::{BaseSource, BaseControl, CollectorControl, CollectorSource, EmitterControl, EmitterSource, AdcControl};

#[embassy_executor::task]
pub async fn runner(r: MeasurementResources) -> ! {
    defmt::debug!("Configuring measurement runner...");

    let (collector_dac, base_dac) = Dac::new_blocking(r.dac, r.collector_dac, r.base_dac).split();
    let base_control = BaseControl::new(r.base_sel1, r.base_sel2, base_dac);
    let collector_control = CollectorControl::new(r.collector_sel1, r.collector_sel2, collector_dac);
    let emitter_control = EmitterControl::new(r.emitter_sel1, r.emitter_sel2);

    let adc_control = AdcControl::new(r.adc, r.dma, r.base_adc, r.collector_adc);

    defmt::info!("Measurement runner: OK");
    defmt::todo!() // clearly not :)
}

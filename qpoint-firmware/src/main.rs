#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

use embassy_executor::Spawner;

use embassy_stm32::{Peri, peripherals};

mod config;
mod measurement;
pub mod util;

assign_resources::assign_resources! {
    measurement: MeasurementResources {
        dac: DAC1,
        adc: ADC1,
        base_sel1: PB13,
        base_sel2: PB14,
        base_dac: PA5,
        base_adc: PA2,
        collector_sel1: PA1,
        collector_sel2: PA0,
        collector_dac: PA4,
        collector_adc: PA6,
        emitter_sel1: PB12,
        emitter_sel2: PB11,
    },
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(config::hal_config());
    let r = split_resources!(p);

    defmt::info!("Startup: OK");
    defmt::debug!("Spawning tasks...");

    spawner.spawn(defmt::unwrap!(measurement::runner(r.measurement)));

    defmt::info!("Tasks spawned: OK");
    defmt::debug!("Yeilding to the executor...");
}

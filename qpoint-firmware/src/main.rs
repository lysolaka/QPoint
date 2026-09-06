#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

use embassy_executor::Spawner;

mod config;

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let _p = embassy_stm32::init(config::hal_config());

    loop {
        defmt::info!("Hello, world!");
        embassy_time::Timer::after_secs(1).await;
    }
}

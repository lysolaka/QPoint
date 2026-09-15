use embassy_stm32::gpio::Speed;
use embassy_stm32::i2c::{Config, I2c};
use embassy_stm32::time::Hertz;

use display_interface::DisplayError;
use display_interface_i2c::I2CInterface;
use oled_async::Builder;
use oled_async::displays::sh1106::Sh1106_128_64;
use oled_async::prelude::*;

use crate::DisplayResources;

#[embassy_executor::task]
pub async fn driver(r: DisplayResources) -> ! {
    defmt::debug!("Configuring display driver...");
    let mut config = Config::default();
    config.frequency = Hertz::khz(400);
    config.gpio_speed = Speed::VeryHigh;

    let i2c = I2c::new(
        r.i2c,
        r.scl,
        r.sda,
        r.tx_dma,
        r.rx_dma,
        crate::Interrupts,
        config,
    );

    let interface = I2CInterface::new(i2c, 0x3c, 0x40);
    let mut display: GraphicsMode<_, _> = Builder::new(Sh1106_128_64 {})
        .with_rotation(DisplayRotation::Rotate180)
        .connect(interface)
        .into();

    let init: Result<(), DisplayError> = async {
        display.init().await?;
        display.flush().await?;
        Ok(())
    }
    .await;

    if let Err(e) = init {
        defmt::error!("Display driver: {}", e);
        defmt::warn!("The display driver is now useless");
        // this is needed because this function (task) never returns
        loop {
            embassy_futures::yield_now().await;
        }
    } else {
        defmt::info!("Display driver: OK!");
    }

    defmt::todo!()
}

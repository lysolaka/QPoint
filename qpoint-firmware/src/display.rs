use embassy_stm32::gpio::Speed;
use embassy_stm32::i2c::{Config, I2c};
use embassy_stm32::pac::{GPIOB, gpio::vals::Idr};
use embassy_stm32::time::Hertz;

use display_interface::DisplayError;
use display_interface_i2c::I2CInterface;
use oled_async::Builder;
use oled_async::displays::sh1106::Sh1106_128_64;
use oled_async::prelude::*;

use crate::DisplayResources;
use crate::util::color;
use crate::util::display;
use crate::util::display::DisplayUpdate;
use crate::util::sync::DISPLAY_UPDATE_Q;
use crate::util::sync::RGB_LED_S;

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
        display::clear(&mut display)?;

        // HACK: we don't know the initial state of the selection and to avoid a global variable and
        // polling, just directly read the GPIO IDR register.
        let selection = GPIOB.idr().read().idr(9) == Idr::HIGH;
        display::draw_layout(&mut display)?;
        display::draw_selection(&mut display, selection)?;

        display.flush().await?;
        Ok(())
    }
    .await;

    if let Err(e) = init {
        defmt::error!("Display driver: {}", e);
        defmt::warn!("The display driver is now useless");
        RGB_LED_S.signal(color::WARN);
        // this is needed because this function (task) never returns
        loop {
            // void the incoming display updates
            let _ = DISPLAY_UPDATE_Q.receive().await;
        }
    } else {
        defmt::info!("Display driver: OK!");
    }

    loop {
        let update = DISPLAY_UPDATE_Q.receive().await;

        let result: Result<(), DisplayError> = async {
            match update {
                DisplayUpdate::Lock => display::draw_cover(&mut display)?,
                DisplayUpdate::Unlock => {
                    display::clear(&mut display)?;
                    // HACK: we don't know the initial state of the selection and to avoid a global variable and
                    // polling, just directly read the GPIO IDR register.
                    let selection = GPIOB.idr().read().idr(9) == Idr::HIGH;
                    display::draw_layout(&mut display)?;
                    display::draw_selection(&mut display, selection)?;
                }
                DisplayUpdate::ClearValues => display::clear_values(&mut display)?,
                DisplayUpdate::SetSelection(sel) => display::draw_selection(&mut display, sel)?,
                DisplayUpdate::SetUbe(ube) => display::draw_ube(&mut display, ube)?,
                DisplayUpdate::SetHfe(hfe) => display::draw_hfe(&mut display, hfe)?,
                DisplayUpdate::SetUceSat(uce_sat) => display::draw_uce_sat(&mut display, uce_sat)?,
            }

            display.flush().await?;
            Ok(())
        }
        .await;

        if let Err(e) = result {
            defmt::error!("Display update failed: {}", e);
            RGB_LED_S.signal(color::ERR);
        }
    }
}

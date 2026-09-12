//! User interface driver.

use embassy_stm32::exti::ExtiInput;
use embassy_stm32::gpio::{Level, Pull};

use embassy_futures::select::{Either, select};

use crate::UiResources;
use crate::util::sync::ATTACHED_M;

#[embassy_executor::task]
pub async fn driver(r: UiResources) -> ! {
    defmt::debug!("Configuring UI driver...");

    let mut start_bt = ExtiInput::new(r.start_pin, r.start_exti, Pull::None, crate::Interrupts);
    let mut select_bt = ExtiInput::new(r.select_pin, r.select_exti, Pull::None, crate::Interrupts);

    defmt::info!("UI driver: OK!");

    loop {
        let pressed = select(
            start_bt.wait_for_rising_edge(),
            select_bt.wait_for_any_edge(),
        )
        .await;

        if *ATTACHED_M.lock().await {
            defmt::warn!("A button was pressed, but the device is attached. Ignoring...");
            continue;
        }

        match pressed {
            Either::First(_) => {
                defmt::info!("Starting measurement from the local UI");
                let selection = select_bt.get_level();
                if selection == Level::Low {
                    defmt::debug!("Transistor type selection: NPN");
                    // TODO: run measurements for NPN
                } else {
                    defmt::debug!("Transistor type selection: PNP");
                    // TODO: run measurements for PNP
                }
            }
            Either::Second(_) => {
                // TODO: change the transistor type selection on the display
            }
        }
    }
}

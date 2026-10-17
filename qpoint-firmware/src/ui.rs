//! User interface driver.

use embassy_stm32::exti::ExtiInput;
use embassy_stm32::gpio::{Level, Pull};

use embassy_futures::select::{Either, select};
use embassy_time::Timer;

use qpoint_common::Command;
use qpoint_common::measurement::{BaseSource, CollectorSource, EmitterSource};

use crate::UiResources;
use crate::util::color;
use crate::util::display::DisplayUpdate;
use crate::util::measurement::{estimate_uce_sat, interval};
use crate::util::sync::ATTACHED_M;
use crate::util::sync::DISPLAY_UPDATE_Q;
use crate::util::sync::MEASUREMENT_CMD_Q;
use crate::util::sync::RESPONSE_UI_S;
use crate::util::sync::RGB_LED_S;
use crate::util::{CommandSource, MeasurementCommand};

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
                RGB_LED_S.signal(color::WAIT);
                if selection == Level::Low {
                    defmt::debug!("Transistor type selection: NPN");
                    measure_npn().await;
                } else {
                    defmt::debug!("Transistor type selection: PNP");
                    measure_pnp().await;
                }
                // give it some time to rest
                disconnect_sources().await;
                Timer::after_secs(1).await;
                RGB_LED_S.signal(color::OK);
            }
            Either::Second(_) => {
                DISPLAY_UPDATE_Q
                    .send(DisplayUpdate::SetSelection(select_bt.get_level().into()))
                    .await;
            }
        }
    }
}

async fn disconnect_sources() {
    let seq = [
        Command::BaseSelect(BaseSource::HighZ),
        Command::CollectorSelect(CollectorSource::HighZ),
        Command::EmitterSelect(EmitterSource::HighZ),
    ];
    execute_commands(seq.into_iter()).await;
}

async fn measure_npn() {
    // initial conditions for U_BE
    let init = [
        Command::BaseSelect(BaseSource::VSource),
        Command::CollectorSelect(CollectorSource::HighZ),
        Command::EmitterSelect(EmitterSource::GND),
    ];
    execute_commands(init.into_iter()).await;

    RESPONSE_UI_S.reset();
    execute_commands(
        [Command::BaseSet {
            value: 5.0,
            measure: true,
        }]
        .into_iter(),
    )
    .await;

    let result = RESPONSE_UI_S.wait().await;
    let u_be = result.base_voltage / 1000.0;
    defmt::debug!("U_BE = {=f32}", &u_be);

    DISPLAY_UPDATE_Q.send(DisplayUpdate::SetDrop(u_be)).await;

    // initial conditions for h_FE
    let init = [
        Command::BaseSelect(BaseSource::ISource),
        Command::CollectorSelect(CollectorSource::VCC),
        Command::EmitterSelect(EmitterSource::GND),
    ];
    execute_commands(init.into_iter()).await;

    RESPONSE_UI_S.reset();
    execute_commands(
        [Command::BaseSet {
            value: 100.0,
            measure: true,
        }]
        .into_iter(),
    )
    .await;

    let result = RESPONSE_UI_S.wait().await;
    let h_fe = result.collector_current / 0.1;
    defmt::debug!("h_FE = {=f32}", &h_fe);

    DISPLAY_UPDATE_Q.send(DisplayUpdate::SetGain(h_fe)).await;

    // initial conditions for U_CE(sat)
    let init = [
        Command::BaseSelect(BaseSource::ISource),
        Command::BaseSet {
            value: 20.0,
            measure: false,
        },
        Command::CollectorSelect(CollectorSource::VSource),
        Command::EmitterSelect(EmitterSource::GND),
    ];
    execute_commands(init.into_iter()).await;

    RESPONSE_UI_S.reset();
    let mut currents = [0.0; 21];
    let mut step_index = 0;
    for v in interval::<21>(0.0, 0.02) {
        execute_commands(
            [Command::CollectorSet {
                value: v,
                measure: true,
            }]
            .into_iter(),
        )
        .await;
        let result = RESPONSE_UI_S.wait().await;

        currents[step_index] = result.collector_current;
        step_index += 1;
    }

    let u_ce_sat = estimate_uce_sat(currents);
    defmt::debug!("U_CE(sat) = {:?}", &u_ce_sat);

    // TODO: display it
}

async fn measure_pnp() {
    // initial conditions for U_BE
    let init = [
        Command::BaseSelect(BaseSource::VSource),
        Command::CollectorSelect(CollectorSource::HighZ),
        Command::EmitterSelect(EmitterSource::VCC),
    ];
    execute_commands(init.into_iter()).await;

    RESPONSE_UI_S.reset();
    execute_commands(
        [Command::BaseSet {
            value: 0.0,
            measure: true,
        }]
        .into_iter(),
    )
    .await;

    let result = RESPONSE_UI_S.wait().await;
    let u_be = (result.base_voltage / 1000.0) - 5.0;
    defmt::debug!("U_BE = {=f32}", &u_be);

    DISPLAY_UPDATE_Q.send(DisplayUpdate::SetDrop(u_be)).await;

    // initial conditions for h_FE
    let init = [
        Command::BaseSelect(BaseSource::ISink),
        Command::CollectorSelect(CollectorSource::GND),
        Command::EmitterSelect(EmitterSource::VCC),
    ];
    execute_commands(init.into_iter()).await;

    RESPONSE_UI_S.reset();
    execute_commands(
        [Command::BaseSet {
            value: 100.0,
            measure: true,
        }]
        .into_iter(),
    )
    .await;

    let result = RESPONSE_UI_S.wait().await;
    let h_fe = -result.collector_current / 0.1;
    defmt::debug!("h_FE = {=f32}", &h_fe);

    DISPLAY_UPDATE_Q.send(DisplayUpdate::SetGain(h_fe)).await;

    // initial conditions for U_CE(sat)
    let init = [
        Command::BaseSelect(BaseSource::ISink),
        Command::BaseSet {
            value: 40.0,
            measure: false,
        },
        Command::CollectorSelect(CollectorSource::VSource),
        Command::CollectorSet {
            value: 5.0,
            measure: false,
        },
        Command::EmitterSelect(EmitterSource::VCC),
    ];
    execute_commands(init.into_iter()).await;

    RESPONSE_UI_S.reset();
    let mut currents = [0.0; 21];
    let mut step_index = 0;
    for v in interval::<21>(4.6, 0.02).into_iter().rev() {
        execute_commands(
            [Command::CollectorSet {
                value: v,
                measure: true,
            }]
            .into_iter(),
        )
        .await;
        let result = RESPONSE_UI_S.wait().await;

        currents[step_index] = -result.collector_current;
        step_index += 1;
    }

    let u_ce_sat = estimate_uce_sat(currents).map(|v| -v);
    defmt::debug!("U_CE(sat) = {:?}", &u_ce_sat);

    // TODO: display it
}

async fn execute_commands(seq: impl Iterator<Item = Command>) {
    for cmd in seq {
        MEASUREMENT_CMD_Q
            .send(MeasurementCommand {
                cmd,
                source: CommandSource::UI,
            })
            .await;
    }
}

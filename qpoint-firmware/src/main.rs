#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

use embassy_executor::Spawner;

use embassy_stm32::{Peri, peripherals};
use embassy_stm32::{dma, exti, i2c, usb};

mod communication;
mod config;
mod display;
mod led;
mod measurement;
mod ui;
pub mod util;

use crate::util::color;
use crate::util::sync::RGB_LED_S;

assign_resources::assign_resources! {
    display: DisplayResources {
        i2c: I2C2,
        tx_dma: DMA1_CH2,
        rx_dma: DMA1_CH3,
        scl: PB3,
        sda: PB4,
    },
    led: LedResources {
        r: PB6,
        g: PB7,
        b: PB8,
        tim: TIM4,
    },
    measurement: MeasurementResources {
        dac: DAC1,
        adc: ADC1,
        dma: DMA1_CH1,
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
    ui: UiResources {
        start_pin: PB5,
        start_exti: EXTI5,
        select_pin: PB9,
        select_exti: EXTI9,
    }
    usb: UsbResources {
        usb: USB,
        dp: PA12,
        dm: PA11,
    }
}

embassy_stm32::bind_interrupts!(struct Interrupts {
    DMA1_CHANNEL1 => dma::InterruptHandler<peripherals::DMA1_CH1>;
    DMA1_CHANNEL2_3 => dma::InterruptHandler<peripherals::DMA1_CH2>, dma::InterruptHandler<peripherals::DMA1_CH3>;
    EXTI4_15 => exti::InterruptHandler<embassy_stm32::interrupt::typelevel::EXTI4_15>;
    I2C2_3 => i2c::EventInterruptHandler<peripherals::I2C2>, i2c::ErrorInterruptHandler<peripherals::I2C2>;
    USB_UCPD1_2 => usb::InterruptHandler<peripherals::USB>;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(config::hal_config());
    let r = split_resources!(p);

    defmt::info!("Startup: OK!");
    defmt::debug!("Spawning tasks...");

    spawner.spawn(defmt::unwrap!(led::driver(r.led)));
    spawner.spawn(defmt::unwrap!(ui::driver(r.ui)));
    spawner.spawn(defmt::unwrap!(display::driver(r.display)));
    spawner.spawn(defmt::unwrap!(measurement::runner(r.measurement)));

    communication::start(spawner, r.usb);

    RGB_LED_S.signal(color::OK);

    defmt::info!("Tasks spawned: OK!");
    defmt::debug!("Yeilding to the executor...");
}

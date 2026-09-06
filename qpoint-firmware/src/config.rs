use embassy_stm32::rcc::{AHBPrescaler, APBPrescaler};
use embassy_stm32::rcc::{Hse, HseMode, Sysclk};
use embassy_stm32::rcc::{
    Pll, PllMul, PllPreDiv, PllQDiv, PllRDiv, PllSource, VoltageRange, mux::Usbsel,
};
use embassy_stm32::time::Hertz;

pub fn hal_config() -> embassy_stm32::Config {
    let mut c = embassy_stm32::Config::default();

    c.rcc.hse = Some(Hse {
        freq: Hertz::mhz(16),
        mode: HseMode::Bypass,
    });
    c.rcc.sys = Sysclk::PLL1_R;
    c.rcc.pll = Some(Pll {
        source: PllSource::HSE,
        prediv: PllPreDiv::DIV1,
        mul: PllMul::MUL12,
        divp: None,
        divq: Some(PllQDiv::DIV4),
        divr: Some(PllRDiv::DIV3),
    });
    c.rcc.ahb_pre = AHBPrescaler::DIV1;
    c.rcc.apb1_pre = APBPrescaler::DIV1;

    c.rcc.mux.usbsel = Usbsel::PLL1_Q;

    c.rcc.voltage_range = VoltageRange::RANGE1;

    c.enable_ucpd1_dead_battery = false;
    c.enable_ucpd2_dead_battery = false;

    c.enable_debug_during_sleep = true;

    c
}

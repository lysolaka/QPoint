//! RGB LED driver.

use embassy_stm32::gpio::OutputType;
use embassy_stm32::time::Hertz;
use embassy_stm32::timer::low_level::{CountingMode, OutputPolarity};
use embassy_stm32::timer::simple_pwm::{PwmPin, SimplePwm};

use crate::LedResources;
use crate::util::sync::RGB_LED_S;

#[embassy_executor::task]
pub async fn driver(r: LedResources) -> ! {
    defmt::debug!("Configuring RGB LED driver...");
    let red = PwmPin::new(r.r, OutputType::OpenDrain);
    let green = PwmPin::new(r.g, OutputType::OpenDrain);
    let blue = PwmPin::new(r.b, OutputType::OpenDrain);

    let mut led = SimplePwm::new(
        r.tim,
        Some(red),
        Some(green),
        Some(blue),
        None,
        Hertz::hz(62_500),
        CountingMode::EdgeAlignedUp,
    );

    defmt::debug!("RGB LED max_duty_cycle: {=u32}", &led.max_duty_cycle());

    led.ch1().set_polarity(OutputPolarity::ActiveLow);
    led.ch2().set_polarity(OutputPolarity::ActiveLow);
    led.ch3().set_polarity(OutputPolarity::ActiveLow);
    led.ch1().enable();
    led.ch2().enable();
    led.ch3().enable();

    defmt::info!("LED Driver: OK!");

    loop {
        let (r, g, b) = RGB_LED_S.wait().await;
        defmt::debug!(
            "Setting LED colour to: ({=u8:02x}, {=u8:02x}, {=u8:02x})",
            r,
            g,
            b
        );
        led.ch1().set_duty_cycle_fraction(r as u32, 255);
        led.ch2().set_duty_cycle_fraction(g as u32, 255);
        led.ch3().set_duty_cycle_fraction(b as u32, 255);
    }
}

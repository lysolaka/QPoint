use embassy_executor::Spawner;

use embassy_stm32::peripherals::USB;
use embassy_stm32::usb::Driver;
use embassy_usb::class::cdc_acm::{BufferedReceiver, CdcAcmClass, Sender, State};
use embassy_usb::{Builder, Config, UsbDevice};

use static_cell::{ConstStaticCell, StaticCell};

use crate::UsbResources;
use crate::util::color;
use crate::util::sync::RGB_LED_S;

mod receiver;
mod sender;

static CONFIG_DESCRIPTOR_BUF: ConstStaticCell<[u8; 256]> = ConstStaticCell::new([0; 256]);
static BOS_DESCRIPTOR_BUF: ConstStaticCell<[u8; 256]> = ConstStaticCell::new([0; 256]);
static CONTROL_BUF: ConstStaticCell<[u8; 64]> = ConstStaticCell::new([0; 64]);

static CDC_ACM_STATE: StaticCell<State<'static>> = StaticCell::new();
static CDC_ACM_RX_BUF: ConstStaticCell<[u8; 64]> = ConstStaticCell::new([0; 64]);

pub fn start(spawner: Spawner, r: UsbResources) {
    defmt::debug!("Configuring communication interface...");
    let driver = Driver::new(r.usb, crate::Interrupts, r.dp, r.dm);

    let mut config = Config::new(0x6769, 0xf420);
    config.manufacturer = Some("Engine");
    config.product = Some("QPoint");
    config.serial_number = Some("2/5");
    // config.max_power = 500;

    let mut builder = Builder::new(
        driver,
        config,
        CONFIG_DESCRIPTOR_BUF.take(),
        BOS_DESCRIPTOR_BUF.take(),
        &mut [],
        CONTROL_BUF.take(),
    );

    let class = CdcAcmClass::new(&mut builder, CDC_ACM_STATE.init(State::new()), 64);
    let (sender, receiver) = class.split();
    let receiver = receiver.into_buffered(CDC_ACM_RX_BUF.take());

    let usb = builder.build();

    spawner.spawn(defmt::unwrap!(runner(usb)));
    spawner.spawn(defmt::unwrap!(serial_receiver(receiver)));
    spawner.spawn(defmt::unwrap!(serial_sender(sender)));
}

#[embassy_executor::task]
async fn runner(mut usb: UsbDevice<'static, Driver<'static, USB>>) -> ! {
    defmt::info!("USB Device: OK!");
    usb.run().await
}

#[embassy_executor::task]
async fn serial_receiver(mut receiver: BufferedReceiver<'static, Driver<'static, USB>>) -> ! {
    defmt::info!("CDC ACM Receiver: OK!");

    loop {
        receiver.wait_connection().await;
        defmt::info!("CDC ACM RX connected");
        match receiver::run(&mut receiver).await {
            Err(e) => defmt::error!("{:?}", defmt::Display2Format(&e)),
            _ => defmt::unreachable!(),
        }
        RGB_LED_S.signal(color::ERR);
    }
}

#[embassy_executor::task]
async fn serial_sender(mut sender: Sender<'static, Driver<'static, USB>>) -> ! {
    defmt::info!("CDC ACM Sender: OK!");

    loop {
        sender.wait_connection().await;
        defmt::info!("CDC ACM TX connected");
        match sender::run(&mut sender).await {
            Err(e) => defmt::error!("{:?}", defmt::Display2Format(&e)),
            _ => defmt::unreachable!(),
        }
        RGB_LED_S.signal(color::ERR);
    }
}

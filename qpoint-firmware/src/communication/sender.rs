use core::convert::Infallible;

use embassy_stm32::peripherals::USB;
use embassy_stm32::usb::Driver;
use embassy_usb::class::cdc_acm::Sender;

use embedded_io_async::Write;

use crate::util::sync::RESPONSE_TX_Q;
use crate::util::com;

pub async fn run<'d>(sender: &mut Sender<'d, Driver<'d, USB>>) -> Result<Infallible, com::Error> {
    loop {
        let msg = RESPONSE_TX_Q.receive().await;
        let mut buf = [0; 32];

        let data = postcard::to_slice_cobs(&msg, &mut buf)?;
        sender.write_all(data).await?;
    }
}

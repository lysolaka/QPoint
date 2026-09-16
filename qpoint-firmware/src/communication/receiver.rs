use core::convert::Infallible;

use embassy_stm32::peripherals::USB;
use embassy_stm32::usb::Driver;
use embassy_usb::class::cdc_acm::BufferedReceiver;

use qpoint_common::{Command, Response};

use crate::util::com;
use crate::util::com::FramedReceiver;
use crate::util::display::DisplayUpdate;
use crate::util::sync::{DISPLAY_UPDATE_Q, MEASUREMENT_CMD_Q, RESPONSE_TX_Q, RGB_LED_S};
use crate::util::{CommandSource, MeasurementCommand};

pub async fn run<'d>(
    receiver: &mut BufferedReceiver<'d, Driver<'d, USB>>,
) -> Result<Infallible, com::Error> {
    let mut receiver = FramedReceiver::new(receiver);
    loop {
        let frame = receiver.read_frame().await?;
        defmt::trace!("Got frame {=[u8]:02x}", &frame);

        let cmd: Command = postcard::from_bytes_cobs(frame)?;
        defmt::trace!("Deserialized to: {:?}", &cmd);

        defmt::debug!("Running command: {:?}", &cmd);
        match cmd {
            Command::Attach => {
                DISPLAY_UPDATE_Q.send(DisplayUpdate::SetAttached).await;
                RESPONSE_TX_Q.send(Response::Ok).await;
            }
            Command::Detach => {
                DISPLAY_UPDATE_Q.send(DisplayUpdate::SetNormal).await;
                RESPONSE_TX_Q.send(Response::Ok).await;
            }
            Command::DisplayParam { param, value } => {
                DISPLAY_UPDATE_Q
                    .send(DisplayUpdate::SetParameter { param, value })
                    .await;
                RESPONSE_TX_Q.send(Response::Ok).await;
            }
            Command::LedSet(r, g, b) => {
                RGB_LED_S.signal((r, g, b));
                RESPONSE_TX_Q.send(Response::Ok).await;
            }
            cmd => {
                let cmd = MeasurementCommand {
                    cmd,
                    source: CommandSource::Remote,
                };
                MEASUREMENT_CMD_Q.send(cmd).await;
            }
        }
    }
}

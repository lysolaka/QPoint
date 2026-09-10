use core::convert::Infallible;

use embassy_stm32::peripherals::USB;
use embassy_stm32::usb::Driver;
use embassy_usb::class::cdc_acm::BufferedReceiver;

use qpoint_common::Command;

use crate::util::com;
use crate::util::com::FramedReceiver;
use crate::util::sync::MEASUREMENT_CMD_Q;
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
            Command::Attach => defmt::todo!(),
            Command::Detach => defmt::todo!(),
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

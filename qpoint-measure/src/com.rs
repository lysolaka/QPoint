use std::time::Duration;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio_serial::{ClearBuffer, SerialPort, SerialPortBuilderExt, SerialPortType, SerialStream};

use qpoint_common::{Command, Response};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    SerialPort(#[from] tokio_serial::Error),
    #[error("{0}")]
    Serialization(#[from] postcard::Error),
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("The device is not found")]
    NotFound,
}

#[derive(Debug)]
pub struct Device {
    port: BufReader<SerialStream>,
}

impl Device {
    pub async fn attach() -> Result<Self, Error> {
        let device = tokio_serial::available_ports()?
            .into_iter()
            .find(|port| {
                matches!(
                    &port.port_type,
                    SerialPortType::UsbPort(info) if info.vid == 0x6769 && info.pid == 0xf420
                )
            })
            .map(|port| port.port_name)
            .ok_or(Error::NotFound)?;

        let mut port = tokio_serial::new(&device, 115200)
            .timeout(Duration::from_millis(500))
            .open_native_async()?;

        let attach_cmd = postcard::to_stdvec_cobs(&Command::Lock)?;
        port.write_all(&attach_cmd).await?;

        Ok(Self {
            port: BufReader::new(port),
        })
    }

    pub async fn detach(&mut self) -> Result<(), Error> {
        self.port.get_mut().clear(ClearBuffer::All)?;
        let detach_cmd = postcard::to_stdvec_cobs(&Command::Unlock)?;
        self.port.write_all(&detach_cmd).await?;

        Ok(())
    }

    pub async fn transaction(&mut self, cmd: Command) -> Result<Response, Error> {
        eprintln!("Sending: {:?}", &cmd);
        let cmd = postcard::to_stdvec_cobs(&cmd)?;
        self.port.write_all(&cmd).await?;

        let mut response = Vec::with_capacity(128);
        self.port.read_until(0x00, &mut response).await?;

        let response = postcard::from_bytes_cobs(&mut response)?;
        eprintln!("Received: {:?}", &response);

        Ok(response)
    }
}

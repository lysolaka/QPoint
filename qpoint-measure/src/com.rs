use std::time::Duration;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio_serial::{SerialPortBuilderExt, SerialPortType, SerialStream};

use qpoint_common::{Command, Response};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    SerialPort(#[from] tokio_serial::Error),
    #[error("{0}")]
    Serialization(#[from] postcard::Error),
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("The device was not found")]
    NotFound,
    #[error("Unexpected response from the device")]
    UnexpectedResponse,
}

#[derive(Debug)]
pub struct Device {
    port: BufReader<SerialStream>,
}

impl Device {
    pub fn connect() -> Result<Self, Error> {
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

        let port = tokio_serial::new(&device, 115200)
            .timeout(Duration::from_millis(500))
            .open_native_async()?;

        Ok(Self {
            port: BufReader::new(port),
        })
    }

    // TODO: acquire a semaphore on lock and release on unlock
    pub async fn lock(&mut self) -> Result<(), Error> {
        let r = self.transaction(Command::Lock).await?;
        match r {
            Response::Ack => Ok(()),
            _ => Err(Error::UnexpectedResponse)
        }
    }

    pub async fn unlock(&mut self) -> Result<(), Error> {
        let r = self.transaction(Command::Unlock).await?;
        match r {
            Response::Ack => Ok(()),
            _ => Err(Error::UnexpectedResponse)
        }
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

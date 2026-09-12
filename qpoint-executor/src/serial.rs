use std::io::{BufRead, BufReader};
use std::time::Duration;

use serialport::SerialPortType;

use qpoint_common::{Command, Response};

pub fn execute(cmd: Command) -> Result<Response, crate::Error> {
                    eprintln!("Executing: {:?}", cmd);

    let device = serialport::available_ports()?
        .into_iter()
        .find(|port| {
            matches!(
                &port.port_type,
                SerialPortType::UsbPort(info)
                    if info.vid == 0x6769 && info.pid == 0xf420
            )
        })
        .map(|port| port.port_name)
        .ok_or(crate::Error::NotFound)?;

    eprintln!("Found device: {}", &device);

    let mut port = serialport::new(&device, 115200)
        .timeout(Duration::from_millis(500))
        .open()?;

    let cmd = postcard::to_stdvec_cobs(&cmd)?;
    eprintln!("Writing {:02x?}", &cmd);
    port.write_all(&cmd)?;

    let mut port = BufReader::with_capacity(128, port);
    let mut response = Vec::with_capacity(128);
    port.read_until(0x00, &mut response)?;
    eprintln!("Read: {:02x?}", &response);

    let response = postcard::from_bytes_cobs(&mut response)?;
    eprintln!("Response: {:?}", response);
    Ok(response)
}

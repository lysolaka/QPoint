use embassy_usb::class::cdc_acm::CdcAcmError;

use embedded_io_async::Read;

/// Error type used in communication code.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("USB host disconnected")]
    Disconnected(#[from] CdcAcmError),
    #[error("Serialization error `{0}`")]
    Serialization(#[from] postcard::Error),
    #[error("CDC ACM Received EOF")]
    ReceiveEOF,
}

/// Framed receiver for a COBS encoded data stream.
pub struct FramedReceiver<T: Read> {
    rx: T,
    frame_buf: [u8; 64],
    frame_len: usize,
}

impl<T: Read> FramedReceiver<T>
where
    Error: From<T::Error>,
{
    /// Construct a new [`FramedReceiver`].
    pub fn new(rx: T) -> Self {
        Self {
            rx,
            frame_buf: [0; 64],
            frame_len: 0,
        }
    }

    /// Read a single frame from the receiver. The frame is COBS encoded and includes a `0x00` byte
    /// at the end.
    pub async fn read_frame(&mut self) -> Result<&mut [u8], Error> {
        let mut rx_buf = [0; 64];

        loop {
            let n = self.rx.read(&mut rx_buf).await?;
            defmt::trace!("Read {=usize} bytes from RX", n);
            if n == 0 {
                return Err(Error::ReceiveEOF);
            }

            for &b in &rx_buf[..n] {
                if self.frame_len >= self.frame_buf.len() {
                    defmt::warn!("Receive buffer overflow, flushing");
                    self.frame_len = 0;
                }

                self.frame_buf[self.frame_len] = b;
                self.frame_len += 1;

                if b == 0x00 {
                    let frame = &mut self.frame_buf[..self.frame_len];
                    self.frame_len = 0;
                    return Ok(frame);
                }
            }
        }
    }
}

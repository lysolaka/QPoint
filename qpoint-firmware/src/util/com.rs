use embassy_usb::class::cdc_acm::CdcAcmError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("USB host disconnected")]
    Disconnected(#[from] CdcAcmError),
    #[error("Serialization error `{0}`")]
    Serialization(#[from] postcard::Error),
}

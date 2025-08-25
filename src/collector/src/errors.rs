use thiserror::Error;

#[derive(Debug, Error)]
pub enum CollectorError {
    #[error("Unable to connect to the server")]
    UnableToConnect,
    #[error("Unable to send data")]
    UnableToSend,
    #[error("Unable to receive data")]
    UnableToReceive,
}
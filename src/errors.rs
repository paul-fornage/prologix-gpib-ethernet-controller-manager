use std::num::ParseIntError;
use std::str::Utf8Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GpibControllerError {
    #[error("Error trying to send or receive data: {0}")]
    TcpIoError(#[from] std::io::Error),
    #[error("error parsing integer from a string: {0}")]
    ParseIntError(#[from] ParseIntError),
    #[error("error parsing string from TCPStream: {0}")]
    ParseStringError(#[from] Utf8Error),
    #[error("Buffer overflow. Increase the size of the buffer or reconsider what you're doing")]
    BufferTooSmall,
}
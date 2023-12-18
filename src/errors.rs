use std::any::Any;
use std::error::Error;
use std::num::ParseIntError;
use std::string::FromUtf8Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BatTestError {
    #[error("Error trying to send or receive data: {0}")]
    TcpIoError(#[from] std::io::Error),
    #[error("error parsing integer from a string: {0}")]
    ParseIntError(#[from] ParseIntError),
    #[error("error parsing string from TCPStream: {0}")]
    ParseStringError(#[from] FromUtf8Error),
    #[error("Error parsing url")]
    UrlParseError
}
use quinn::{ConnectionError, ReadToEndError, WriteError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NetworkError {
    #[error("IO Error")]
    IOError(#[from] std::io::Error),
    #[error("Fatal Error {0}")]
    FatalError(String),
    #[error("TLS Error")]
    TLSError(#[from] rustls::Error),
    #[error("Connection Error")]
    ConnectionError(#[from] ConnectionError),
    #[error("Connection Error")]
    ConnectError(#[from] quinn::ConnectError),
    #[error("Connection Closed Error{0}")]
    ConnectionClosedError(String),
    #[error("RequestError {0}")]
    RequestError(String),
    #[error("Stream Read Error: {0}")]
    RecvError(String),
    #[error("Internal {0}")]
    InternalError(String),
    #[error("unknownerror {0}")]
    Unknown(String),
    #[error("Write Error")]
    WriteError(#[from] WriteError),
}

impl From<Box<dyn std::error::Error>> for NetworkError {
    fn from(value: Box<dyn std::error::Error>) -> Self {
        return Self::InternalError(value.to_string());
    }
}

impl From<ReadToEndError> for NetworkError {
    fn from(value: ReadToEndError) -> Self {
        return Self::RecvError(value.to_string());
    }
}

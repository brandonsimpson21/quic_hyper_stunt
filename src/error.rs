use thiserror::Error;

#[derive(Error, Debug)]
pub enum NetworkError {
    #[error("IO Error")]
    IOError(#[from] std::io::Error),
    #[error("Fatal Error {0}")]
    FatalError(String),
    #[error("TLS Error")]
    TLSError(#[from] rustls::Error),
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
    #[error("Hyper Error")]
    HyperError(#[from] hyper::Error),
}

impl From<Box<dyn std::error::Error>> for NetworkError {
    fn from(value: Box<dyn std::error::Error>) -> Self {
        return Self::InternalError(value.to_string());
    }
}

impl From<anyhow::Error> for NetworkError {
    fn from(value: anyhow::Error) -> Self {
        return Self::InternalError(value.to_string());
    }
}

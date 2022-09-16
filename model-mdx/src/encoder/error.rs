use thiserror::Error;

/// Errors that occur while encoding MDX models to bytes
#[derive(Debug, Error)]
pub enum Error {
    #[error("")]
    Dummy,
}
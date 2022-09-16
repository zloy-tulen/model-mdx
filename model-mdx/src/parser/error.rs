use thiserror::Error;

/// Errors that occur while decoding MDX models from bytes
#[derive(Debug, Error)]
pub enum Error {
    #[error("")]
    Dummy,
}
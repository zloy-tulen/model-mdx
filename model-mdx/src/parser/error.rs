use nom::error::{ContextError, ErrorKind, ParseError};
use std::fmt;
use thiserror::Error;

/// Errors that occur while decoding MDX models from bytes
#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Parsing(String),
    #[error("Input stream is incomplete, needed: {0:?}")]
    Incomplete(nom::Needed),
}

// Errors that occur while decoding MDX models from bytes
#[derive(Debug, Error)]
pub enum MdxParseError<I: fmt::Debug> {
    /// Parser error (not enough input and e.t.c)
    #[error("Error {1:?} at: {0:?}")]
    Nom(I, ErrorKind),
    /// Added context for debug.
    #[error("Context: {0}. Error: {1}")]
    Context(String, Box<Self>),
    /// Raised when we expect specific type of chunk, but the tag
    /// in the file differs.
    #[error("Expected chunk {0}, but got {1}")]
    UnexpectedChunkTag(String, String),
    /// Raised when we try to fetch fixed size string, but there is not 
    /// enough bytes.
    #[error("Not enough input for dixed size string, expected {expected}, but got {found}")]
    TooShortLiteral { expected: usize, found: usize },
    #[error("Failed to convert string literal: {0}")]
    Utf8Conv(#[from] std::str::Utf8Error),
}

impl<'a> From<(&'a [u8], ErrorKind)> for MdxParseError<&'a [u8]> {
    fn from((input, kind): (&'a [u8], ErrorKind)) -> Self {
        MdxParseError::Nom(input, kind)
    }
}

impl<'a> ParseError<&'a [u8]> for MdxParseError<&'a [u8]> {
    fn from_error_kind(input: &'a [u8], kind: ErrorKind) -> Self {
        MdxParseError::Nom(input, kind)
    }

    fn append(_: &[u8], _: ErrorKind, other: Self) -> Self {
        other
    }
}

impl<'a> ContextError<&'a [u8]> for MdxParseError<&'a [u8]> {
    fn add_context(_input: &'a [u8], ctx: &'static str, other: Self) -> Self {
        MdxParseError::Context(ctx.to_owned(), Box::new(other))
    }
}

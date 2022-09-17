pub use crate::encoder::error::Error as EncodeError;
pub use crate::parser::Parser;

/// Types that can be parsed and encoded to bytes
pub trait Materialized: Sized {
    /// Parse the chunk from given input
    fn parse(input: &[u8]) -> Parser<Self>;

    /// Encode the chunk to byte stream
    fn encode(&self, output: &mut Vec<u8>) -> Result<(), EncodeError>;
}

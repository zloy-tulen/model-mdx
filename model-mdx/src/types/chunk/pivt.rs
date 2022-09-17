use super::Chunk;
use crate::encoder::error::Error as EncodeError;
use crate::parser::Parser;
use crate::types::materialize::Materialized;
use super::utils::Tag;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pivt {}

impl Chunk for Pivt {
    fn tag() -> Tag {
        Tag([0x50, 0x49, 0x56, 0x54]) // PIVT
    }
}

impl Materialized for Pivt {
    fn parse(input: &[u8]) -> Parser<Self> {
        unimplemented!();
    }

    fn encode(&self, output: &mut Vec<u8>) -> Result<(), EncodeError> {
        unimplemented!();
    }
}

use super::Chunk;
use crate::encoder::error::Error as EncodeError;
use crate::parser::Parser;
use crate::types::materialize::Materialized;
use super::utils::Tag;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cams {}

impl Chunk for Cams {
    fn tag() -> Tag {
        Tag([0x43, 0x41, 0x4d, 0x53]) // CAMS
    }
}

impl Materialized for Cams {
    fn parse(input: &[u8]) -> Parser<Self> {
        unimplemented!();
    }

    fn encode(&self, output: &mut Vec<u8>) -> Result<(), EncodeError> {
        unimplemented!();
    }
}
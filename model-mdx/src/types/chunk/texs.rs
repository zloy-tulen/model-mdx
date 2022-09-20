use super::Chunk;
use crate::encoder::error::Error as EncodeError;
use crate::parser::Parser;
use crate::types::materialize::Materialized;
use super::utils::Tag;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Texs {}

impl Chunk for Texs {
    fn tag() -> Tag {
        Tag([0x54, 0x45, 0x58, 0x53]) // TEXS
    }
}

impl Materialized for Texs {
    type Version = u32;

    fn parse_versioned(_: Option<Self::Version>, input: &[u8]) -> Parser<Self> {
        unimplemented!();
    }

    fn encode(&self, output: &mut Vec<u8>) -> Result<(), EncodeError> {
        unimplemented!();
    }
}
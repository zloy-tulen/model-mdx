use super::Chunk;
use crate::encoder::error::Error as EncodeError;
use crate::parser::Parser;
use crate::types::materialize::Materialized;
use super::utils::Tag;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Mtls {}

impl Chunk for Mtls {
    fn tag() -> Tag {
        Tag([0x4d, 0x54, 0x4c, 0x53]) // MTLS
    }
}

impl Materialized for Mtls {
    fn parse(input: &[u8]) -> Parser<Self> {
        unimplemented!();
    }

    fn encode(&self, output: &mut Vec<u8>) -> Result<(), EncodeError> {
        unimplemented!();
    }
}
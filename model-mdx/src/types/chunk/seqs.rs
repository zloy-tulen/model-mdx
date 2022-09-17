use super::Chunk;
use crate::encoder::error::Error as EncodeError;
use crate::parser::Parser;
use crate::types::materialize::Materialized;
use super::utils::Tag;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Seqs {}

impl Chunk for Seqs {
    fn tag() -> Tag {
        Tag([0x53, 0x45, 0x51, 0x53]) // SEQS
    }
}

impl Materialized for Seqs {
    fn parse(input: &[u8]) -> Parser<Self> {
        unimplemented!();
    }

    fn encode(&self, output: &mut Vec<u8>) -> Result<(), EncodeError> {
        unimplemented!();
    }
}

use super::utils::Tag;
use super::Chunk;
use crate::encoder::error::Error as EncodeError;
use crate::parser::Parser;
use crate::types::materialize::Materialized;
use crate::types::sequence::{Sequence, SEQUENCE_SIZE};
use log::*;
use nom::{error::context, multi::count};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Seqs {
    pub sequences: Vec<Sequence>,
}

impl Chunk for Seqs {
    fn tag() -> Tag {
        Tag([0x53, 0x45, 0x51, 0x53]) // SEQS
    }
}

impl Materialized for Seqs {
    type Version = u32;

    fn parse_versioned(_: Option<Self::Version>, input: &[u8]) -> Parser<Self> {
        let (input, header) = context("SEQS header", Self::expect_header)(input)?;
        if header.size % SEQUENCE_SIZE != 0 {
            warn!("SEQS chunk contains not whole count of sequences!");
        }
        let n = header.size / SEQUENCE_SIZE;
        let (input, sequences) = context("sequences", count(Sequence::parse, n))(input)?;
        Ok((input, Seqs { sequences }))
    }

    fn encode(&self, output: &mut Vec<u8>) -> Result<(), EncodeError> {
        self.encode_header(4, output)?;
        for s in self.sequences.iter() {
            s.encode(output)?;
        }
        Ok(())
    }
}

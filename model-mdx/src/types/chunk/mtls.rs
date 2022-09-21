use super::utils::Tag;
use super::Chunk;
use crate::encoder::error::Error as EncodeError;
use crate::parser::primitives::parse_all;
use crate::parser::Parser;
use crate::types::{material::Material, materialize::Materialized};
use nom::error::context;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Mtls {
    pub materials: Vec<Material>,
}

impl Chunk for Mtls {
    fn tag() -> Tag {
        Tag([0x4d, 0x54, 0x4c, 0x53]) // MTLS
    }
}

impl Materialized for Mtls {
    type Version = u32;

    fn parse_versioned(version: Option<Self::Version>, input: &[u8]) -> Parser<Self> {
        let (input, _) = context("MTLS header", Self::expect_header)(input)?;
        let (input, materials) = context(
            "materials",
            parse_all(|input| Material::parse_versioned(version, input)),
        )(input)?;
        Ok((input, Mtls { materials }))
    }

    fn encode(&self, output: &mut Vec<u8>) -> Result<(), EncodeError> {
        self.encode_header(4, output)?;
        for v in self.materials.iter() {
            v.encode(output)?;
        }
        Ok(())
    }
}

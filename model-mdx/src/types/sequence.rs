use super::extent::Extent;
use super::materialize::{EncodeError, Materialized, Parser};
use super::utils::*;
use crate::parser::primitives::{le_f32, times};
use nom::{error::context, number::complete::le_u32};

pub const SEQUENCE_SIZE: usize = 132;
pub const SEQUENCE_NAME_LENGTH: usize = 80;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Sequence {
    pub name: Literal<SEQUENCE_NAME_LENGTH>,
    pub interval: [u32; 2],
    pub move_speed: f32,
    pub flags: u32, // 1 looping, 0 not looping
    pub rarity: f32,
    pub sync_point: u32,
    pub extent: Extent,
}

impl Materialized for Sequence {
    /// Parse the chunk from given input
    fn parse(input: &[u8]) -> Parser<Self> {
        let (input, name) = context("name", Literal::parse)(input)?;
        let (input, interval) = context("interval", times::<2, u32, _>(le_u32))(input)?;
        let (input, move_speed) = context("moveSpeed", le_f32)(input)?;
        let (input, flags) = context("flags", le_u32)(input)?;
        let (input, rarity) = context("rarity", le_f32)(input)?;
        let (input, sync_point) = context("syncPoint", le_u32)(input)?;
        let (input, extent) = context("extent", Extent::parse)(input)?;
        Ok((
            input,
            Sequence {
                name,
                interval,
                move_speed,
                flags,
                rarity,
                sync_point,
                extent,
            },
        ))
    }

    /// Encode the chunk to byte stream
    fn encode(&self, output: &mut Vec<u8>) -> Result<(), EncodeError> {
        self.name.encode(output)?;
        for i in self.interval {
            output.extend(i.to_le_bytes());
        }
        output.extend(self.move_speed.to_le_bytes());
        output.extend(self.flags.to_le_bytes());
        output.extend(self.rarity.to_le_bytes());
        output.extend(self.sync_point.to_le_bytes());
        self.extent.encode(output)?;
        Ok(())
    }
}

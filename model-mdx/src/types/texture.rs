use super::chunk::utils::*;
use super::materialize::*;

/// Amount of bytes one texture record takes
pub const TEXTURE_SIZE: usize = 268;
/// Length of `file_name` field of [Texture]
pub const TEXTURE_FILENAME_LEN: usize = 260;

// Texture {
//     uint32 replaceableId
//     char[260] fileName
//     uint32 flags
//   }
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Texture {
    pub replaceable_id: u32,
    pub file_name: Literal<TEXTURE_FILENAME_LEN>,
    pub flags: u32,
}

impl Materialized for Texture {
    type Version = u32;

    fn parse_versioned(_: Option<Self::Version>, input: &[u8]) -> Parser<Self> {
        let (input, replaceable_id) = context("replaceable_id", Materialized::parse)(input)?;
        let (input, file_name) = context("file_name", Materialized::parse)(input)?;
        let (input, flags) = context("flags", Materialized::parse)(input)?;
        Ok((
            input,
            Texture {
                replaceable_id,
                file_name,
                flags,
            },
        ))
    }

    fn encode(&self, output: &mut Vec<u8>) -> Result<(), EncodeError> {
        self.replaceable_id.encode(output)?;
        self.file_name.encode(output)?;
        self.flags.encode(output)
    }
}

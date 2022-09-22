pub mod animation;
pub mod chunk;
pub mod extent;
pub mod geoset;
pub mod layer;
pub mod material;
pub mod materialize;
pub mod sequence;
#[cfg(test)]
mod tests;
pub mod texture;
pub mod tracks;

use super::encoder::error::Error as EncodeError;
use super::parser::error::Error as ParseError;
pub use chunk::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct MdxModel {
    pub root: Mdlx,
}

impl MdxModel {
    pub fn from_slice(slice: &[u8]) -> Result<Self, ParseError> {
        super::parser::parse_mdx(slice)
    }

    pub fn to_vec(&self) -> Result<Vec<u8>, EncodeError> {
        super::encoder::encode_mdx(self)
    }
}

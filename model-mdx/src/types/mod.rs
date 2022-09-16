#[cfg(test)]
mod tests;

use super::parser::error::Error as ParseError;
use super::encoder::error::Error as EncodeError;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MdxModel {

}

impl MdxModel {
    pub fn from_slice(slice: &[u8]) -> Result<Self, ParseError> {
        super::parser::parse_mdx(slice)
    } 

    pub fn to_vec(&self) -> Result<Vec<u8>, EncodeError> {
        super::encoder::encode_mdx(self)
    }
}
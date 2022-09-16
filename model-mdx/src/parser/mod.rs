pub mod error;

use error::Error;
use crate::types::MdxModel;

/// Parse MDX model from input bytes
pub fn parse_mdx(input: &[u8]) -> Result<MdxModel, Error> {
    unimplemented!();
}
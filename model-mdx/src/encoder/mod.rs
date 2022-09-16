pub mod error;

use error::Error;
use crate::types::MdxModel;

/// Encode MDX model into bytes
pub fn encode_mdx(model: &MdxModel) -> Result<Vec<u8>, Error> {
    unimplemented!();
}
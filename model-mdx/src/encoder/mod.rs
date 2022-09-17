pub mod error;
pub(crate) mod primitives;

use crate::types::{materialize::Materialized, MdxModel};
use error::Error;

/// Encode MDX model into bytes
pub fn encode_mdx(model: &MdxModel) -> Result<Vec<u8>, Error> {
    let mut output = vec![];
    model.root.encode(&mut output)?;
    Ok(output)
}

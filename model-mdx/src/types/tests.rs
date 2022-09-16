use super::*;
use include_dir::{include_dir, Dir};
use log::*;
use test_log::test;

static ASSETS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../assets");

fn mdx_encode_decode(input: &[u8]) {
    let model = MdxModel::from_slice(input).expect("parsed");
    let encoded = model.to_vec().expect("encoded");
    assert_eq!(input, &encoded, "Input is not equal encoded!");
}

#[test]
fn test_encode_decode_ident() {
    for entry in ASSETS_DIR.find("**/*.mdx").unwrap() {
        if let Some(file) = entry.as_file() {
            info!("Testing {}", file.path().display());
            mdx_encode_decode(file.contents());
        }
    }
    for entry in ASSETS_DIR.find("**/*.MDX").unwrap() {
        if let Some(file) = entry.as_file() {
            info!("Testing {}", file.path().display());
            mdx_encode_decode(file.contents());
        }
    }
}

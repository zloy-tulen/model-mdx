pub mod atch;
pub mod bone;
pub mod bpos;
pub mod cams;
pub mod clid;
pub mod corn;
pub mod evts;
pub mod fafx;
pub mod geoa;
pub mod geos;
pub mod glbs;
pub mod help;
pub mod lite;
pub mod mdlx;
pub mod modl;
pub mod mtls;
pub mod pivt;
pub mod pre2;
pub mod prem;
pub mod ribb;
pub mod seqs;
pub mod snds;
pub mod texs;
pub mod txan;
pub mod utils;
pub mod vers;

pub use atch::*;
pub use bone::*;
pub use bpos::*;
pub use cams::*;
pub use clid::*;
pub use corn::*;
pub use evts::*;
pub use fafx::*;
pub use geoa::*;
pub use geos::*;
pub use glbs::*;
pub use help::*;
pub use lite::*;
pub use mdlx::*;
pub use modl::*;
pub use mtls::*;
pub use pivt::*;
pub use pre2::*;
pub use prem::*;
pub use ribb::*;
pub use seqs::*;
pub use snds::*;
pub use texs::*;
pub use txan::*;
pub use utils::*;
pub use vers::*;

use super::utils::Tag;
use crate::encoder::error::Error as EncodeError;
use crate::parser::Parser;
use crate::types::materialize::Materialized;
use nom::{error::context, combinator::peek};

/// MDX file consists of hierarchy of chunks. They are started with
/// known tags of 4 ASCII characters and size. There are many types
/// of chunks, some has predefined size in elements, some are not.
pub trait Chunk: Sized + Materialized {
    /// Fixed tag for given type of chunk
    fn tag() -> Tag;

    /// Parse header and guard that the tag matches with that is
    /// returned by [tag] function
    fn expect_header(input: &[u8]) -> Parser<Header> {
        Header::expect(Self::tag(), input)
    }

    /// Parse tag only and guard that the tag matches with that is
    /// returned by [tag] function
    fn expect_tag(input: &[u8]) -> Parser<Tag> {
        Tag::expect(Self::tag(), input)
    }

    /// Write down header with given size of body
    fn encode_header(&self, size: usize, output: &mut Vec<u8>) -> Result<(), EncodeError> {
        let header = Header {
            tag: Self::tag(),
            size,
        };
        header.encode(output)
    }

    /// Write down only chunk header tag
    fn encode_tag(&self, output: &mut Vec<u8>) -> Result<(), EncodeError> {
        Self::tag().encode(output)
    }
}

/// Parse headers of chunks and pass them into user specified handler. 
/// The combinator does this until input is not empty.
pub fn parse_subchunks<F>(mut body: F) -> impl FnOnce(&[u8]) -> Parser<()> 
where 
F: FnMut(Header, &[u8]) -> Parser<()> 
{
    move |input| {
        let mut cycle_input: &[u8] = input;
        while !cycle_input.is_empty() {
            let (input, header): (&[u8], Header) = context("subchunk header", peek(Materialized::parse))(cycle_input)?;
            let (input, _) = body(header, input)?;
            cycle_input = input;
        }
        Ok((cycle_input, ()))
    }
}
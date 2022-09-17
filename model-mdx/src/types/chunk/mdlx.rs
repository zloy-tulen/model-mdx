use super::utils::*;
use super::Chunk;
use crate::encoder::error::Error as EncodeError;
use crate::parser::Parser;
use crate::types::materialize::Materialized;
use log::*;
use nom::{bytes::complete::take, combinator::peek, error::context};
use super::utils::Tag;

use super::{
    atch::*, bone::*, bpos::*, cams::*, clid::*, corn::*, evts::*, fafx::*, geoa::*, geos::*,
    glbs::*, help::*, lite::*, modl::*, mtls::*, pivt::*, pre2::*, prem::*, ribb::*, seqs::*,
    snds::*, texs::*, txan::*, vers::*,
};

/// Root chunk of the hierarchy
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Mdlx {
    pub vers: Option<Vers>,
    pub modl: Option<Modl>,
    pub seqs: Option<Seqs>,
    pub glbs: Option<Glbs>,
    pub texs: Option<Texs>,
    pub snds: Option<Snds>,
    pub mtls: Option<Mtls>,
    pub txan: Option<Txan>,
    pub geos: Option<Geos>,
    pub geoa: Option<Geoa>,
    pub bone: Option<Bone>,
    pub lite: Option<Lite>,
    pub help: Option<Help>,
    pub atch: Option<Atch>,
    pub pivt: Option<Pivt>,
    pub prem: Option<Prem>,
    pub pre2: Option<Pre2>,
    pub ribb: Option<Ribb>,
    pub evts: Option<Evts>,
    pub cams: Option<Cams>,
    pub clid: Option<Clid>,
    // The following chunks are for version > 800
    pub bpos: Option<Bpos>,
    pub fafx: Option<Fafx>,
    pub corn: Option<Corn>,
}

impl Mdlx {
    pub fn new() -> Self {
        Mdlx {
            vers: None,
            modl: None,
            seqs: None,
            glbs: None,
            texs: None,
            snds: None,
            mtls: None,
            txan: None,
            geos: None,
            geoa: None,
            bone: None,
            lite: None,
            help: None,
            atch: None,
            pivt: None,
            prem: None,
            pre2: None,
            ribb: None,
            evts: None,
            cams: None,
            clid: None,
            bpos: None,
            fafx: None,
            corn: None,
        }
    }
}

impl Chunk for Mdlx {
    fn tag() -> Tag {
        Tag([0x4d, 0x44, 0x4c, 0x58]) // MDLX
    }
}

impl Materialized for Mdlx {
    fn parse(input: &[u8]) -> Parser<Self> {
        let (input, _) = context("MDLX tag", Self::expect_tag)(input)?;

        let mut cycle_input = input;
        let mut result = Self::new();
        while !cycle_input.is_empty() {
            let (input, Header { tag, size }) =
                context("chunk header", peek(Header::parse))(cycle_input)?;
            let found: String = std::str::from_utf8(&tag.0)
                .map(|s| s.to_owned())
                .unwrap_or_else(|_| format!("{:?}", tag.0));
            trace!("Found chunk with tag {} and size {}", found, size);
            if tag == Vers::tag() {
                let (input, chunk) = context("VERS chunk", Vers::parse)(input)?;
                result.vers = Some(chunk);
                cycle_input = input;
            } else if tag == Modl::tag() {
                let (input, chunk) = context("MODL chunk", Modl::parse)(input)?;
                result.modl = Some(chunk);
                cycle_input = input;
            } else if tag == Seqs::tag() {
                let (input, chunk) = context("SEQS chunk", Seqs::parse)(input)?;
                result.seqs = Some(chunk);
                cycle_input = input;
            } else if tag == Glbs::tag() {
                let (input, chunk) = context("GLBS chunk", Glbs::parse)(input)?;
                result.glbs = Some(chunk);
                cycle_input = input;
            } else if tag == Texs::tag() {
                let (input, chunk) = context("TEXS chunk", Texs::parse)(input)?;
                result.texs = Some(chunk);
                cycle_input = input;
            } else if tag == Snds::tag() {
                let (input, chunk) = context("SNDS chunk", Snds::parse)(input)?;
                result.snds = Some(chunk);
                cycle_input = input;
            } else if tag == Mtls::tag() {
                let (input, chunk) = context("MTLS chunk", Mtls::parse)(input)?;
                result.mtls = Some(chunk);
                cycle_input = input;
            } else if tag == Txan::tag() {
                let (input, chunk) = context("TXAN chunk", Txan::parse)(input)?;
                result.txan = Some(chunk);
                cycle_input = input;
            } else if tag == Geos::tag() {
                let (input, chunk) = context("GEOS chunk", Geos::parse)(input)?;
                result.geos = Some(chunk);
                cycle_input = input;
            } else if tag == Geoa::tag() {
                let (input, chunk) = context("GEOA chunk", Geoa::parse)(input)?;
                result.geoa = Some(chunk);
                cycle_input = input;
            } else if tag == Bone::tag() {
                let (input, chunk) = context("BONE chunk", Bone::parse)(input)?;
                result.bone = Some(chunk);
                cycle_input = input;
            } else if tag == Lite::tag() {
                let (input, chunk) = context("LITE chunk", Lite::parse)(input)?;
                result.lite = Some(chunk);
                cycle_input = input;
            } else if tag == Help::tag() {
                let (input, chunk) = context("HELP chunk", Help::parse)(input)?;
                result.help = Some(chunk);
                cycle_input = input;
            } else if tag == Atch::tag() {
                let (input, chunk) = context("ATCH chunk", Atch::parse)(input)?;
                result.atch = Some(chunk);
                cycle_input = input;
            } else if tag == Pivt::tag() {
                let (input, chunk) = context("PIVT chunk", Pivt::parse)(input)?;
                result.pivt = Some(chunk);
                cycle_input = input;
            } else if tag == Prem::tag() {
                let (input, chunk) = context("PREM chunk", Prem::parse)(input)?;
                result.prem = Some(chunk);
                cycle_input = input;
            } else if tag == Pre2::tag() {
                let (input, chunk) = context("PRE2 chunk", Pre2::parse)(input)?;
                result.pre2 = Some(chunk);
                cycle_input = input;
            } else if tag == Ribb::tag() {
                let (input, chunk) = context("RIBB chunk", Ribb::parse)(input)?;
                result.ribb = Some(chunk);
                cycle_input = input;
            } else if tag == Evts::tag() {
                let (input, chunk) = context("EVTS chunk", Evts::parse)(input)?;
                result.evts = Some(chunk);
                cycle_input = input;
            } else if tag == Cams::tag() {
                let (input, chunk) = context("CAMS chunk", Cams::parse)(input)?;
                result.cams = Some(chunk);
                cycle_input = input;
            } else if tag == Clid::tag() {
                let (input, chunk) = context("CLID chunk", Clid::parse)(input)?;
                result.clid = Some(chunk);
                cycle_input = input;
            } else if tag == Bpos::tag() {
                let (input, chunk) = context("BPOS chunk", Bpos::parse)(input)?;
                result.bpos = Some(chunk);
                cycle_input = input;
            } else if tag == Fafx::tag() {
                let (input, chunk) = context("FAFX chunk", Fafx::parse)(input)?;
                result.fafx = Some(chunk);
                cycle_input = input;
            } else if tag == Corn::tag() {
                let (input, chunk) = context("CORN chunk", Corn::parse)(input)?;
                result.corn = Some(chunk);
                cycle_input = input;
            } else {
                trace!("Unknown chunk {:?}, skipping it", &tag);
                let (input, _) = context("skip unknown chunk", take(size))(cycle_input)?;
                cycle_input = input;
            }
        }
        Ok((cycle_input, result))
    }

    fn encode(&self, output: &mut Vec<u8>) -> Result<(), EncodeError> {
        self.encode_tag(output)?;
        if let Some(chunk) = &self.vers {
            chunk.encode(output)?;
        }
        if let Some(chunk) = &self.modl {
            chunk.encode(output)?;
        }
        if let Some(chunk) = &self.seqs {
            chunk.encode(output)?;
        }
        if let Some(chunk) = &self.glbs {
            chunk.encode(output)?;
        }
        if let Some(chunk) = &self.texs {
            chunk.encode(output)?;
        }
        if let Some(chunk) = &self.snds {
            chunk.encode(output)?;
        }
        if let Some(chunk) = &self.mtls {
            chunk.encode(output)?;
        }
        if let Some(chunk) = &self.txan {
            chunk.encode(output)?;
        }
        if let Some(chunk) = &self.geos {
            chunk.encode(output)?;
        }
        if let Some(chunk) = &self.geoa {
            chunk.encode(output)?;
        }
        if let Some(chunk) = &self.bone {
            chunk.encode(output)?;
        }
        if let Some(chunk) = &self.lite {
            chunk.encode(output)?;
        }
        if let Some(chunk) = &self.help {
            chunk.encode(output)?;
        }
        if let Some(chunk) = &self.atch {
            chunk.encode(output)?;
        }
        if let Some(chunk) = &self.pivt {
            chunk.encode(output)?;
        }
        if let Some(chunk) = &self.prem {
            chunk.encode(output)?;
        }
        if let Some(chunk) = &self.pre2 {
            chunk.encode(output)?;
        }
        if let Some(chunk) = &self.ribb {
            chunk.encode(output)?;
        }
        if let Some(chunk) = &self.evts {
            chunk.encode(output)?;
        }
        if let Some(chunk) = &self.cams {
            chunk.encode(output)?;
        }
        if let Some(chunk) = &self.clid {
            chunk.encode(output)?;
        }
        if let Some(chunk) = &self.bpos {
            chunk.encode(output)?;
        }
        if let Some(chunk) = &self.fafx {
            chunk.encode(output)?;
        }
        if let Some(chunk) = &self.corn {
            chunk.encode(output)?;
        }
        Ok(())
    }
}

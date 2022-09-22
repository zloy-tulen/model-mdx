use super::utils::Tag;
use super::utils::*;
use super::{parse_subchunks, Chunk};
use crate::encoder::error::Error as EncodeError;
use crate::parser::Parser;
use crate::types::materialize::Materialized;
use log::*;
use nom::{bytes::complete::take, error::context};

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
    type Version = u32;

    fn parse_versioned(version_ext: Option<Self::Version>, input: &[u8]) -> Parser<Self> {
        let (input, _) = context("MDLX tag", Self::expect_tag)(input)?;

        let mut result = Self::new();
        let mut version: Option<Self::Version> = version_ext;
        let (input, _) = parse_subchunks(|Header { tag, size }, input| {
            if tag == Vers::tag() {
                let (input, chunk) =
                    context("VERS chunk", |input| Vers::parse_versioned(version, input))(input)?;
                version = Some(chunk.version);
                trace!("Version: {}", chunk.version);
                result.vers = Some(chunk);
                Ok((input, ()))
            } else if tag == Modl::tag() {
                let (input, chunk) =
                    context("MODL chunk", |input| Modl::parse_versioned(version, input))(input)?;
                result.modl = Some(chunk);
                Ok((input, ()))
            } else if tag == Seqs::tag() {
                let (input, chunk) =
                    context("SEQS chunk", |input| Seqs::parse_versioned(version, input))(input)?;
                result.seqs = Some(chunk);
                Ok((input, ()))
            } else if tag == Glbs::tag() {
                let (input, chunk) =
                    context("GLBS chunk", |input| Glbs::parse_versioned(version, input))(input)?;
                result.glbs = Some(chunk);
                Ok((input, ()))
            } else if tag == Texs::tag() {
                let (input, chunk) =
                    context("TEXS chunk", |input| Texs::parse_versioned(version, input))(input)?;
                result.texs = Some(chunk);
                Ok((input, ()))
            } else if tag == Snds::tag() {
                let (input, chunk) =
                    context("SNDS chunk", |input| Snds::parse_versioned(version, input))(input)?;
                result.snds = Some(chunk);
                Ok((input, ()))
            } else if tag == Mtls::tag() {
                let (input, chunk) =
                    context("MTLS chunk", |input| Mtls::parse_versioned(version, input))(input)?;
                result.mtls = Some(chunk);
                Ok((input, ()))
            } else if tag == Txan::tag() {
                let (input, chunk) =
                    context("TXAN chunk", |input| Txan::parse_versioned(version, input))(input)?;
                result.txan = Some(chunk);
                Ok((input, ()))
            } else if tag == Geos::tag() {
                let (input, chunk) =
                    context("GEOS chunk", |input| Geos::parse_versioned(version, input))(input)?;
                result.geos = Some(chunk);
                Ok((input, ()))
            } else if tag == Geoa::tag() {
                let (input, chunk) =
                    context("GEOA chunk", |input| Geoa::parse_versioned(version, input))(input)?;
                result.geoa = Some(chunk);
                Ok((input, ()))
            } else if tag == Bone::tag() {
                let (input, chunk) =
                    context("BONE chunk", |input| Bone::parse_versioned(version, input))(input)?;
                result.bone = Some(chunk);
                Ok((input, ()))
            } else if tag == Lite::tag() {
                let (input, chunk) =
                    context("LITE chunk", |input| Lite::parse_versioned(version, input))(input)?;
                result.lite = Some(chunk);
                Ok((input, ()))
            } else if tag == Help::tag() {
                let (input, chunk) =
                    context("HELP chunk", |input| Help::parse_versioned(version, input))(input)?;
                result.help = Some(chunk);
                Ok((input, ()))
            } else if tag == Atch::tag() {
                let (input, chunk) =
                    context("ATCH chunk", |input| Atch::parse_versioned(version, input))(input)?;
                result.atch = Some(chunk);
                Ok((input, ()))
            } else if tag == Pivt::tag() {
                let (input, chunk) =
                    context("PIVT chunk", |input| Pivt::parse_versioned(version, input))(input)?;
                result.pivt = Some(chunk);
                Ok((input, ()))
            } else if tag == Prem::tag() {
                let (input, chunk) =
                    context("PREM chunk", |input| Prem::parse_versioned(version, input))(input)?;
                result.prem = Some(chunk);
                Ok((input, ()))
            } else if tag == Pre2::tag() {
                let (input, chunk) =
                    context("PRE2 chunk", |input| Pre2::parse_versioned(version, input))(input)?;
                result.pre2 = Some(chunk);
                Ok((input, ()))
            } else if tag == Ribb::tag() {
                let (input, chunk) =
                    context("RIBB chunk", |input| Ribb::parse_versioned(version, input))(input)?;
                result.ribb = Some(chunk);
                Ok((input, ()))
            } else if tag == Evts::tag() {
                let (input, chunk) =
                    context("EVTS chunk", |input| Evts::parse_versioned(version, input))(input)?;
                result.evts = Some(chunk);
                Ok((input, ()))
            } else if tag == Cams::tag() {
                let (input, chunk) =
                    context("CAMS chunk", |input| Cams::parse_versioned(version, input))(input)?;
                result.cams = Some(chunk);
                Ok((input, ()))
            } else if tag == Clid::tag() {
                let (input, chunk) =
                    context("CLID chunk", |input| Clid::parse_versioned(version, input))(input)?;
                result.clid = Some(chunk);
                Ok((input, ()))
            } else if tag == Bpos::tag() {
                let (input, chunk) =
                    context("BPOS chunk", |input| Bpos::parse_versioned(version, input))(input)?;
                result.bpos = Some(chunk);
                Ok((input, ()))
            } else if tag == Fafx::tag() {
                let (input, chunk) =
                    context("FAFX chunk", |input| Fafx::parse_versioned(version, input))(input)?;
                result.fafx = Some(chunk);
                Ok((input, ()))
            } else if tag == Corn::tag() {
                let (input, chunk) =
                    context("CORN chunk", |input| Corn::parse_versioned(version, input))(input)?;
                result.corn = Some(chunk);
                Ok((input, ()))
            } else {
                trace!("Unknown chunk {:?}, skipping it", &tag);
                let (input, _) = context("skip unknown chunk", take(size))(input)?;
                Ok((input, ()))
            }
        })(input)?;

        Ok((input, result))
    }

    fn encode(&self, output: &mut Vec<u8>) -> Result<(), EncodeError> {
        Self::encode_tag(output)?;
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

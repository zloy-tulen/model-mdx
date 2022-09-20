use super::materialize::*;
use super::utils::*;
use crate::types::chunk::*;
use bitflags::bitflags;
use log::*;
use nom::{bytes::complete::take, error::context};
use std::fmt;

// Layer {
//     uint32 inclusiveSize
//     uint32 filterMode // 0: none
//                       // 1: transparent
//                       // 2: blend
//                       // 3: additive
//                       // 4: add alpha
//                       // 5: modulate
//                       // 6: modulate 2x
//     uint32 shadingFlags // 0x1: unshaded
//                         // 0x2: sphere environment map
//                         // 0x4: ?
//                         // 0x8: ?
//                         // 0x10: two sided
//                         // 0x20: unfogged
//                         // 0x40: no depth test
//                         // 0x80: no depth set
//     uint32 textureId
//     uint32 textureAnimationId
//     uint32 coordId
//     float alpha
//     if (version > 800) {
//       float emissiveGain
//       float[3] fresnelColor
//       float fresnelOpacity
//       float fresnelTeamColor
//     }
//     (KMTF)
//     (KMTA)
//     if (version > 800) {
//       (KMTE)
//     }
//     if (version > 900) {
//       (KFC3)
//       (KFCA)
//       (KFTC)
//     }
//   }

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum FilterMode {
    None,
    Transparent,
    Blend,
    Additive,
    AddAlpha,
    Modulate,
    Modulate2x,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct UnknownFilterMode(pub u32);

impl std::error::Error for UnknownFilterMode {}

impl fmt::Display for UnknownFilterMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unknown filter mode {}", self.0)
    }
}

impl TryFrom<u32> for FilterMode {
    type Error = UnknownFilterMode;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(FilterMode::None),
            1 => Ok(FilterMode::Transparent),
            2 => Ok(FilterMode::Blend),
            3 => Ok(FilterMode::Additive),
            4 => Ok(FilterMode::AddAlpha),
            5 => Ok(FilterMode::Modulate),
            6 => Ok(FilterMode::Modulate2x),
            _ => Err(UnknownFilterMode(value)),
        }
    }
}

impl From<FilterMode> for u32 {
    fn from(value: FilterMode) -> u32 {
        match value {
            FilterMode::None => 0,
            FilterMode::Transparent => 1,
            FilterMode::Blend => 2,
            FilterMode::Additive => 3,
            FilterMode::AddAlpha => 4,
            FilterMode::Modulate => 5,
            FilterMode::Modulate2x => 6,
        }
    }
}

impl Materialized for FilterMode {
    type Version = ();

    fn parse_versioned(_: Option<Self::Version>, input: &[u8]) -> Parser<Self> {
        let (input, flag): (&[u8], u32) = Materialized::parse(input)?;
        let filter = flag
            .try_into()
            .map_err(|e: UnknownFilterMode| nom::Err::Failure(e.into()))?;
        Ok((input, filter))
    }

    fn encode(&self, output: &mut Vec<u8>) -> Result<(), EncodeError> {
        let flag: u32 = (*self).into();
        flag.encode(output)
    }
}

bitflags! {
    pub struct ShadingFlags: u32 {
        const Unshaded = 0b00000001;
        const SphereEnvMap = 0b00000010;
        const Unknown4 = 0b00000100;
        const Unknown8 = 0b00001000;
        const TwoSided = 0b00010000;
        const Unfogged = 0b00100000;
        const NoDepthTest = 0b01000000;
        const NoDepthSet = 0b10000000;
    }
}

impl Materialized for ShadingFlags {
    type Version = ();

    fn parse_versioned(_: Option<Self::Version>, input: &[u8]) -> Parser<Self> {
        let (input, flag): (&[u8], u32) = Materialized::parse(input)?;
        Ok((input, ShadingFlags { bits: flag }))
    }

    fn encode(&self, output: &mut Vec<u8>) -> Result<(), EncodeError> {
        self.bits.encode(output)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Layer {
    pub filter_mode: FilterMode,
    pub shading_flags: ShadingFlags,
    pub texture_id: u32,
    pub texture_animation_id: u32,
    pub coord_id: u32,
    pub alpha: f32,
    // if (version > 800)
    pub extra: Option<LayerExt>,
    // all versions
    pub kmtf: Option<Kmtf>,
    pub kmta: Option<Kmta>,
    // if (version > 800)
    pub kmte: Option<Kmte>,
    // if (version > 900)
    pub kfc3: Option<Kfc3>,
    pub kfca: Option<Kfca>,
    pub kftc: Option<Kftc>,
}

impl Materialized for Layer {
    type Version = u32;

    fn parse_versioned(version: Option<Self::Version>, input: &[u8]) -> Parser<Self> {
        parse_inclusive_sized(|input| {
            let (input, filter_mode) = context("filter_mode", Materialized::parse)(input)?;
            let (input, shading_flags) = context("shading_flags", Materialized::parse)(input)?;
            let (input, texture_id) = context("texture_id", Materialized::parse)(input)?;
            let (input, texture_animation_id) =
                context("texture_animation_id", Materialized::parse)(input)?;
            let (input, coord_id) = context("coord_id", Materialized::parse)(input)?;
            let (input, alpha) = context("alpha", Materialized::parse)(input)?;
            let (input, extra) = parse_versioned_greater(
                version,
                800,
                context("layer extra fields", Materialized::parse),
            )(input)?;
            // they can be in any order here and optional
            let mut kmtf: Option<Kmtf> = None;
            let mut kmta: Option<Kmta> = None;
            let mut kmte: Option<Kmte> = None;
            let mut kfc3: Option<Kfc3> = None;
            let mut kfca: Option<Kfca> = None;
            let mut kftc: Option<Kftc> = None;
            parse_subchunks(|header, input| {
                if header.tag == Kmtf::tag() {
                    let (input, chunk) = context("KMTF chunk", Materialized::parse)(input)?;
                    kmtf = Some(chunk);
                    Ok((input, ()))
                } else if header.tag == Kmta::tag() {
                    let (input, chunk) = context("KMTA chunk", Materialized::parse)(input)?;
                    kmta = Some(chunk);
                    Ok((input, ()))
                } else if header.tag == Kmte::tag() {
                    let (input, chunk) = context("KMTE chunk", Materialized::parse)(input)?;
                    kmte = Some(chunk);
                    Ok((input, ()))
                } else if header.tag == Kfc3::tag() {
                    let (input, chunk) = context("KFC3 chunk", Materialized::parse)(input)?;
                    kfc3 = Some(chunk);
                    Ok((input, ()))
                } else if header.tag == Kfca::tag() {
                    let (input, chunk) = context("KFCA chunk", Materialized::parse)(input)?;
                    kfca = Some(chunk);
                    Ok((input, ()))
                } else if header.tag == Kftc::tag() {
                    let (input, chunk) = context("KFTC chunk", Materialized::parse)(input)?;
                    kftc = Some(chunk);
                    Ok((input, ()))
                } else {
                    warn!("Unknown chunk {:?}, skipping it", &header.tag);
                    let (input, _) = context("skip unknown chunk", take(header.size))(input)?;
                    Ok((input, ()))
                }
            })(input)?;
            Ok((
                input,
                Layer {
                    filter_mode,
                    shading_flags,
                    texture_id,
                    texture_animation_id,
                    coord_id,
                    alpha,
                    extra,
                    kmtf,
                    kmta,
                    kmte,
                    kfc3,
                    kfca,
                    kftc,
                },
            ))
        })(input)
    }

    fn encode(&self, output: &mut Vec<u8>) -> Result<(), EncodeError> {
        encode_inclusive_sized(output, |output| {
            self.filter_mode.encode(output)?;
            self.shading_flags.encode(output)?;
            self.texture_id.encode(output)?;
            self.texture_animation_id.encode(output)?;
            self.coord_id.encode(output)?;
            self.alpha.encode(output)?;
            if let Some(v) = &self.extra {
                v.encode(output)?;
            }
            if let Some(v) = &self.kmtf {
                v.encode(output)?;
            }
            if let Some(v) = &self.kmta {
                v.encode(output)?;
            }
            if let Some(v) = &self.kmte {
                v.encode(output)?;
            }
            if let Some(v) = &self.kfc3 {
                v.encode(output)?;
            }
            if let Some(v) = &self.kfca {
                v.encode(output)?;
            }
            if let Some(v) = &self.kftc {
                v.encode(output)?;
            }
            Ok(())
        })
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct LayerExt {
    pub emissive_gain: f32,
    pub fresnel_color: [f32; 3],
    pub fresnel_opacity: f32,
    pub fresnel_team_color: f32,
}

impl Materialized for LayerExt {
    type Version = u32;

    /// Parse the chunk from given input
    fn parse_versioned(_: Option<Self::Version>, input: &[u8]) -> Parser<Self> {
        let (input, emissive_gain) = context("emissive_gain", Materialized::parse)(input)?;
        let (input, fresnel_color) = context("fresnel_color", Materialized::parse)(input)?;
        let (input, fresnel_opacity) = context("fresnel_opacity", Materialized::parse)(input)?;
        let (input, fresnel_team_color) =
            context("fresnel_team_color", Materialized::parse)(input)?;
        Ok((
            input,
            LayerExt {
                emissive_gain,
                fresnel_color,
                fresnel_opacity,
                fresnel_team_color,
            },
        ))
    }

    /// Encode the chunk to byte stream
    fn encode(&self, output: &mut Vec<u8>) -> Result<(), EncodeError> {
        self.emissive_gain.encode(output)?;
        self.fresnel_color.encode(output)?;
        self.fresnel_opacity.encode(output)?;
        self.fresnel_team_color.encode(output)?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Kmtf {
    pub texture_id: u32,
}

impl Chunk for Kmtf {
    fn tag() -> Tag {
        Tag([0x4B, 0x4D, 0x54, 0x46]) // KMTF
    }
}

impl Materialized for Kmtf {
    type Version = ();

    fn parse_versioned(_: Option<Self::Version>, input: &[u8]) -> Parser<Self> {
        let (input, _) = context("KMTF header", Self::expect_header)(input)?;
        let (input, texture_id) = context("texture_id", Materialized::parse)(input)?;
        Ok((input, Kmtf { texture_id }))
    }

    fn encode(&self, output: &mut Vec<u8>) -> Result<(), EncodeError> {
        self.encode_header(4, output)?;
        self.texture_id.encode(output)?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Kmta {
    pub alpha: f32,
}

impl Chunk for Kmta {
    fn tag() -> Tag {
        Tag([0x4B, 0x4D, 0x54, 0x41]) // KMTA
    }
}

impl Materialized for Kmta {
    type Version = ();

    fn parse_versioned(_: Option<Self::Version>, input: &[u8]) -> Parser<Self> {
        let (input, _) = context("KMTA header", Self::expect_header)(input)?;
        let (input, alpha) = context("alpha", Materialized::parse)(input)?;
        Ok((input, Kmta { alpha }))
    }

    fn encode(&self, output: &mut Vec<u8>) -> Result<(), EncodeError> {
        self.encode_header(4, output)?;
        self.alpha.encode(output)?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Kmte {
    pub emissive_gain: f32,
}

impl Chunk for Kmte {
    fn tag() -> Tag {
        Tag([0x4B, 0x4D, 0x54, 0x45]) // KMTE
    }
}

impl Materialized for Kmte {
    type Version = ();

    fn parse_versioned(_: Option<Self::Version>, input: &[u8]) -> Parser<Self> {
        let (input, _) = context("KMTE header", Self::expect_header)(input)?;
        let (input, emissive_gain) = context("emissive_gain", Materialized::parse)(input)?;
        Ok((input, Kmte { emissive_gain }))
    }

    fn encode(&self, output: &mut Vec<u8>) -> Result<(), EncodeError> {
        self.encode_header(4, output)?;
        self.emissive_gain.encode(output)?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Kfc3 {
    pub fresnel_color: [f32; 3],
}

impl Chunk for Kfc3 {
    fn tag() -> Tag {
        Tag([0x4B, 0x46, 0x43, 0x33]) // KFC3
    }
}

impl Materialized for Kfc3 {
    type Version = ();

    fn parse_versioned(_: Option<Self::Version>, input: &[u8]) -> Parser<Self> {
        let (input, _) = context("KFC3 header", Self::expect_header)(input)?;
        let (input, fresnel_color) = context("fresnel_color", Materialized::parse)(input)?;
        Ok((input, Kfc3 { fresnel_color }))
    }

    fn encode(&self, output: &mut Vec<u8>) -> Result<(), EncodeError> {
        self.encode_header(12, output)?;
        self.fresnel_color.encode(output)?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Kfca {
    pub fresnel_alpha: f32,
}

impl Chunk for Kfca {
    fn tag() -> Tag {
        Tag([0x4B, 0x46, 0x43, 0x41]) // KFCA
    }
}

impl Materialized for Kfca {
    type Version = ();

    fn parse_versioned(_: Option<Self::Version>, input: &[u8]) -> Parser<Self> {
        let (input, _) = context("KFCA header", Self::expect_header)(input)?;
        let (input, fresnel_alpha) = context("fresnel_alpha", Materialized::parse)(input)?;
        Ok((input, Kfca { fresnel_alpha }))
    }

    fn encode(&self, output: &mut Vec<u8>) -> Result<(), EncodeError> {
        self.encode_header(4, output)?;
        self.fresnel_alpha.encode(output)?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Kftc {
    pub fresnel_team_color: f32,
}

impl Chunk for Kftc {
    fn tag() -> Tag {
        Tag([0x4B, 0x46, 0x43, 0x41]) // KFTC
    }
}

impl Materialized for Kftc {
    type Version = ();

    fn parse_versioned(_: Option<Self::Version>, input: &[u8]) -> Parser<Self> {
        let (input, _) = context("KFTC header", Self::expect_header)(input)?;
        let (input, fresnel_team_color) =
            context("fresnel_team_color", Materialized::parse)(input)?;
        Ok((input, Kftc { fresnel_team_color }))
    }

    fn encode(&self, output: &mut Vec<u8>) -> Result<(), EncodeError> {
        self.encode_header(4, output)?;
        self.fresnel_team_color.encode(output)?;
        Ok(())
    }
}

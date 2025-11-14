mod ac3;
mod audio;
mod av01;
mod btrt;
mod ccst;
mod colr;
mod eac3;
mod flac;
mod h264;
mod hevc;
mod mp4a;
mod mp4v;
mod opus;
mod pasp;
mod taic;
mod tx3g;
mod uncv;
mod visual;
mod vp9;

pub use ac3::*;
pub use audio::*;
pub use av01::*;
pub use btrt::*;
pub use ccst::*;
pub use colr::*;
pub use eac3::*;
pub use flac::*;
pub use h264::*;
pub use hevc::*;
pub use mp4a::*;
pub use mp4v::*;
pub use opus::*;
pub use pasp::*;
pub use taic::*;
pub use tx3g::*;
pub use uncv::*;
pub use visual::*;
pub use vp9::*;

use crate::*;
use derive_more::From;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Stsd {
    // pub codecs: Vec<Codec>,
    pub sample_entries: Vec<Verbatim>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Verbatim {
    pub data: Vec<u8>,
}

impl Verbatim {
    pub fn kind(&self) -> &[u8; 4] {
        // we skip the box if size is less than 8 bytes at decoding
        // so it's safe to unwrap here
        self.data[4..8].try_into().unwrap()
    }
    pub fn size(&self) -> u32 {
        // we skip the box if size is less than 8 bytes at decoding
        // so it's safe to unwrap here
        // TODO: handle u64?
        u32::from_be_bytes(self.data[0..4].try_into().unwrap())
    }
}

/// Called a "sample entry" in the ISOBMFF specification.
#[derive(Debug, Clone, PartialEq, Eq, From)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Codec {
    // H264
    Avc1(Avc1),

    // HEVC: SPS/PPS/VPS is inline
    Hev1(Hev1),

    // HEVC: SPS/PPS/VPS is in a separate atom
    Hvc1(Hvc1),

    // VP8
    Vp08(Vp08),

    // VP9
    Vp09(Vp09),

    // AV1
    Av01(Av01),

    // AAC
    Mp4a(Mp4a),

    // Mp4v, MPEG-2
    Mp4v(Mp4v),

    // Text
    Tx3g(Tx3g),

    // Opus
    Opus(Opus),

    // Uncompressed video
    Uncv(Uncv),

    // FLAC audio
    Flac(Flac),

    // AC-3 audio
    Ac3(Ac3),

    // EAC-3 audio
    Eac3(Eac3),

    // Unknown
    Unknown(FourCC),
}

impl Decode for Codec {
    fn decode<B: Buf>(buf: &mut B) -> Result<Self> {
        let atom = Any::decode(buf)?;
        Ok(match atom {
            Any::Avc1(atom) => atom.into(),
            Any::Hev1(atom) => atom.into(),
            Any::Hvc1(atom) => atom.into(),
            Any::Vp08(atom) => atom.into(),
            Any::Vp09(atom) => atom.into(),
            Any::Mp4a(atom) => atom.into(),
            Any::Mp4v(atom) => atom.into(),
            Any::Tx3g(atom) => atom.into(),
            Any::Av01(atom) => atom.into(),
            Any::Opus(atom) => atom.into(),
            Any::Uncv(atom) => atom.into(),
            Any::Flac(atom) => atom.into(),
            Any::Ac3(atom) => atom.into(),
            Any::Eac3(atom) => atom.into(),
            Any::Unknown(kind, _) => Self::Unknown(kind),
            _ => return Err(Error::UnexpectedBox(atom.kind())),
        })
    }
}

impl Encode for Codec {
    fn encode<B: BufMut>(&self, buf: &mut B) -> Result<()> {
        match self {
            Self::Unknown(kind) => kind.encode(buf),
            Self::Avc1(atom) => atom.encode(buf),
            Self::Hev1(atom) => atom.encode(buf),
            Self::Hvc1(atom) => atom.encode(buf),
            Self::Vp08(atom) => atom.encode(buf),
            Self::Vp09(atom) => atom.encode(buf),
            Self::Mp4a(atom) => atom.encode(buf),
            Self::Mp4v(atom) => atom.encode(buf),
            Self::Tx3g(atom) => atom.encode(buf),
            Self::Av01(atom) => atom.encode(buf),
            Self::Opus(atom) => atom.encode(buf),
            Self::Uncv(atom) => atom.encode(buf),
            Self::Flac(atom) => atom.encode(buf),
            Self::Ac3(atom) => atom.encode(buf),
            Self::Eac3(atom) => atom.encode(buf),
        }
    }
}

impl AtomExt for Stsd {
    type Ext = ();

    const KIND_EXT: FourCC = FourCC::new(b"stsd");

    fn decode_body_ext<B: Buf>(buf: &mut B, _ext: ()) -> Result<Self> {
        let entry_count = u32::decode(buf)?;
        let mut sample_entries = Vec::new();

        for _ in 0..entry_count {
            if buf.remaining() < 8 {
                return Err(Error::OverDecode(Self::KIND_EXT));
            }
            let size = u32::from_be_bytes(buf.slice(4).try_into().unwrap()) as usize;
            let data = buf.slice(size).to_vec();
            sample_entries.push(Verbatim { data });
            buf.advance(size);
        }

        Ok(Stsd { sample_entries })
    }

    fn encode_body_ext<B: BufMut>(&self, buf: &mut B) -> Result<()> {
        (self.sample_entries.len() as u32).encode(buf)?;
        for entry in &self.sample_entries {
            buf.append_slice(&entry.data);
        }
        Ok(())
    }
}

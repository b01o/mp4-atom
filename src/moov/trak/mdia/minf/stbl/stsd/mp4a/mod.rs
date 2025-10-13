use crate::*;

// We're trying not to pollute the global namespace
pub mod esds;
pub use esds::Esds;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Mp4a {
    pub audio: Audio,
    pub esds: Esds,
    pub btrt: Option<Btrt>,
    pub taic: Option<Taic>,
}

impl Atom for Mp4a {
    const KIND: FourCC = FourCC::new(b"mp4a");

    fn decode_body<B: Buf>(buf: &mut B) -> Result<Self> {
        let audio = Audio::decode(buf)?;

        let mut btrt = None;
        let mut esds = None;
        let mut taic = None;

        // Find esds in mp4a or wave
        while let Some(atom) = Any::decode_maybe(buf)? {
            match atom {
                Any::Btrt(atom) => btrt = atom.into(),
                Any::Esds(atom) => esds = atom.into(),
                Any::Taic(atom) => taic = atom.into(),
                _ => tracing::warn!("unknown atom: {:?}", atom),
            }
        }

        Ok(Mp4a {
            audio,
            esds: esds.ok_or(Error::MissingBox(Esds::KIND))?,
            btrt,
            taic,
        })
    }

    fn encode_body<B: BufMut>(&self, buf: &mut B) -> Result<()> {
        self.audio.encode(buf)?;
        self.esds.encode(buf)?;
        if self.btrt.is_some() {
            self.btrt.encode(buf)?;
        }

        Ok(())
    }
}

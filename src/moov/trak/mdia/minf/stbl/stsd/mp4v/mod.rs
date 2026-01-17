use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Mp4v {
    pub visual: Visual,
    pub esds: Esds,
    pub btrt: Option<Btrt>,
    pub taic: Option<Taic>,
}

impl Atom for Mp4v {
    const KIND: FourCC = FourCC::new(b"mp4v");

    fn decode_body<B: Buf>(buf: &mut B) -> Result<Self> {
        let visual = Visual::decode(buf)?;

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

        Ok(Mp4v {
            visual,
            esds: esds.ok_or(Error::MissingBox(Esds::KIND))?,
            btrt,
            taic,
        })
    }

    fn encode_body<B: BufMut>(&self, buf: &mut B) -> Result<()> {
        self.visual.encode(buf)?;
        self.esds.encode(buf)?;
        if self.btrt.is_some() {
            self.btrt.encode(buf)?;
        }

        Ok(())
    }
}

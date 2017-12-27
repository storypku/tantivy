use common::BinarySerializable;
use std::io::{self, Read, Write};

mod termdict;
mod streamer;
mod delta_encoder;

pub use self::delta_encoder::{DeltaTermInfo, TermInfoDeltaDecoder, TermInfoDeltaEncoder};
pub use self::delta_encoder::{TermDeltaDecoder, TermDeltaEncoder};

pub use self::streamer::TermStreamerBuilderImpl;
pub use self::streamer::TermStreamerImpl;
pub use self::termdict::TermDictionaryBuilderImpl;
pub use self::termdict::TermDictionaryImpl;

#[derive(Debug)]
pub struct CheckPoint {
    pub stream_offset: u32,
    pub postings_offset: u32,
    pub positions_offset: u32,
}

impl BinarySerializable for CheckPoint {
    fn serialize<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.stream_offset.serialize(writer)?;
        self.postings_offset.serialize(writer)?;
        self.positions_offset.serialize(writer)?;
        Ok(())
    }

    fn deserialize<R: Read>(reader: &mut R) -> io::Result<Self> {
        let stream_offset = u32::deserialize(reader)?;
        let postings_offset = u32::deserialize(reader)?;
        let positions_offset = u32::deserialize(reader)?;
        Ok(CheckPoint {
            stream_offset,
            postings_offset,
            positions_offset,
        })
    }
}

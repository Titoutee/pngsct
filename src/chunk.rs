use crate::chunk_type::ChunkType;
use crate::Result as R;
use crc::Crc;
pub struct Chunk {
    length: u32,  // number of bytes in the chunk's data field
    type_: ChunkType, // 4-byte chunk type code
    data: Vec<u8>, // data bytes appropriate to the chunk type
    crc: u32, /*4-byte CRC calculated on the preceding bytes in the chunk,
    including the chunk type code and chunk data fields, but not including the length field*/ 
}

impl Chunk {
    fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        todo!()
    }
    fn length(&self) -> u32 {
        self.length
    }
    fn chunk_type(&self) -> &ChunkType {
        &self.type_
    }
    fn data(&self) -> &[u8] {
        self.data.as_slice()
    }
    fn crc(&self) -> u32 {
        self.crc
    }
    fn data_as_string(&self) -> R<String> {
        todo!()
    }
    fn as_bytes(&self) -> Vec<u8> {
        todo!()
    }

}
impl TryFrom<&[u8]> for Chunk {
    type Error = crate::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        todo!()
    }
}
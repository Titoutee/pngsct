use crate::chunk_type::ChunkType;
use crate::utils::{Result};
use crc::Crc;
use std::fmt::Display;

#[derive(Debug)]
//Any error that can occur at chunk creation
pub enum ChunkError {
    SliceSizeError, // Tryfrom trait implementation associated error (when checking the fed slice length)
    CRC,            // Wrong CRC at creation (specified is not what the real is)
}

pub struct Chunk {
    chunk_type: ChunkType, // Type of a chunk (4-bytes code)
    data: Vec<u8>,         //Data bytes
}

impl Chunk {
    // Bytes quantities constants
    const DATA_LEN_BYTES: usize = 4;
    const CHUNK_TYPE_BYTES: usize = 4;
    const CRC_BYTES: usize = 4;

    const TOTAL_BYTES: usize = Self::DATA_LEN_BYTES + Self::CHUNK_TYPE_BYTES + Self::CRC_BYTES;
    // Example of u8 slice for constructing a Chunk: [0, 0, 0, 1(length), 97, 97, 97, 97(c_t), ]

    fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        Self { chunk_type, data }
    }
    fn length(&self) -> u32 {
        self.data().len() as u32
    }
    fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }
    fn data(&self) -> &[u8] {
        self.data.as_slice() // Complete slice of the inner data vector
    }
    fn crc(&self) -> u32 {
        let crc = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
        let mut c_type_bytes = self.chunk_type().bytes().to_vec();
        let mut data_bytes: Vec<u8> = self.data().try_into().unwrap();
        c_type_bytes.append(&mut data_bytes);
        crc.checksum(&c_type_bytes)
    }
    fn data_as_string(&self) -> Result<String> {
        Ok(self.data().into_iter().map(|&num| num as char).collect())
    }
    fn as_bytes(&self) -> Vec<u8> {
        let length_bytes = self.length().to_be_bytes();
        let c_type_bytes = self.chunk_type().bytes();
        let data_bytes: Vec<u8> = self.data().try_into().unwrap();
        let crc_bytes = self.crc().to_be_bytes();

        length_bytes
            .into_iter()
            .chain(c_type_bytes.into_iter())
            .chain(data_bytes.into_iter())
            .chain(crc_bytes.into_iter())
            .collect()
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = ChunkError;

    fn try_from(bytes: &[u8]) -> std::result::Result<Self, Self::Error> {
        if bytes.len() < Self::TOTAL_BYTES {
            // Minimum length of a construct slice
            return Err(ChunkError::SliceSizeError);
        }

        // Data length
        let (data_length, remaining) = bytes.split_at(Self::DATA_LEN_BYTES); // This consumes a certain amount of bytes of the total slice from the head
        let data_length =
            u32::from_be_bytes(data_length.try_into().unwrap() /*&[u8] -> [u8; 4]*/);

        // Chunk type
        let (c_type, remaining) = remaining.split_at(Self::CHUNK_TYPE_BYTES);
        let c_type = ChunkType::try_from(
            <[u8; 4]>::try_from(c_type).unwrap(), /*&[u8] -> [u8; 4]*/
        )
        .unwrap();

        // Data
        let (data, remaining) = remaining.split_at(data_length as usize); // If length is zero this does nothing
        let data: Vec<u8> = data.try_into().unwrap(); // &[u8] -> Vec<u8>

        // CRC
        let crc = u32::from_be_bytes(
            remaining.split_at(Self::CRC_BYTES).0.try_into().unwrap(), /*&[u8] -> [u8; 4]*/
        );

        let chunk = Chunk::new(c_type, data);

        //Is the crc valid?
        if chunk.crc() == crc {
            Ok(chunk)
        } else {
            Err(ChunkError::CRC) // If not the creation has to be aborted
        }
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}, {}, {:?}, {}",
            self.length(),
            self.chunk_type(),
            self.data(),
            self.crc()
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}

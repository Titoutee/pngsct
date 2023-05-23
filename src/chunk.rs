use crate::chunk_type::ChunkType;
use crate::utils::{Error, Result};
use crc;
use std::fmt::Display;
use std::str::from_utf8;

#[derive(Debug)]
pub struct Chunk {
    chunk_type: ChunkType, // Type of a chunk (4-bytes code)
    data: Vec<u8>,         //Data bytes
                           // Length and CRC don't have to be part of the type, and are exterior data which can be inferred
                           // This lightens the type definition
}

impl Chunk {
    // Bytes quantities constants
    pub const DATA_LEN_BYTES: usize = 4; // Number of len bytes
    pub const CHUNK_TYPE_BYTES: usize = 4; // Number of type bytes
    pub const CRC_BYTES: usize = 4; // Number of crc bytes
                                    // Data bytes number is omitted as infered by the actual number that `data_len` represents

    /// Data len, type and CRC bytes
    pub const TOTAL_BYTES: usize = Self::DATA_LEN_BYTES + Self::CHUNK_TYPE_BYTES + Self::CRC_BYTES;
    // Example of u8 slice for constructing a Chunk: [0, 0, 0, 1(length), 97, 97, 97, 97(c_t), ]

    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        Self { chunk_type, data }
    }
    pub fn length(&self) -> u32 {
        self.data().len() as u32
    }
    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }
    pub fn data(&self) -> &[u8] {
        self.data.as_slice() // Complete slice of the inner data vector
    }
    pub fn crc(&self) -> u32 {
        let crc = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
        let mut c_type_bytes = self.chunk_type().bytes().to_vec();
        let mut data_bytes: Vec<u8> = self.data().try_into().unwrap();
        c_type_bytes.append(&mut data_bytes);
        crc.checksum(&c_type_bytes)
    }
    pub fn data_as_string(&self) -> Result<String> {
        Ok(from_utf8(self.data()).map_err(|_| Error::InvalidChar)?.to_string())
    }
    /// Chunk as a sequence of bytes (same infos order as for building from bytes)
    pub fn as_bytes(&self) -> Vec<u8> {
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
    type Error = Error;

    /// Takes a sequence of bytes, and transform it into a data length (4 bytes), a type (4 bytes), some data (of length `length` (radix 10 representation)) and the crc (which is just checked, not taken as is)
    fn try_from(bytes: &[u8]) -> std::result::Result<Self, Self::Error> {
        // First check
        if bytes.len() < Self::TOTAL_BYTES {
            // Minimum length of a construct slice (data may be empty)
            return Err(Error::ChunkSliceSizeError);
        }

        // Data length
        let (data_length, remaining) = bytes.split_at(Self::DATA_LEN_BYTES); // This consumes a certain amount of bytes of the total slice from the head
        let data_length =
            u32::from_be_bytes(data_length.try_into().unwrap() /*&[u8] -> [u8; 4]*/);

        // Second check (is data the actual correct size represented by `data_len`? if not slice is rejected)
        if remaining.len() != Self::CHUNK_TYPE_BYTES + Self::CRC_BYTES + data_length as usize
        /*Data has already been consumed from the slice*/
        {
            return Err(Error::ChunkSliceSizeError);
        }

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
            Ok(chunk) // We have here a valid and from bytes built Chunk
        } else {
            Err(Error::CRC) // If not the creation has to be aborted
        }
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}, {}, [some lengthy data sequence], {}",
            self.length(),
            self.chunk_type(),
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
    fn test_valid_length() {
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
        let chunk = Chunk::try_from(chunk_data.as_slice()).unwrap();
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

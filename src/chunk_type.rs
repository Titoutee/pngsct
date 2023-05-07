use crate::Error;
use std::{
    fmt::Display,
    str::{self, from_utf8, FromStr},
};

// Any error occuring with the creation of a ChunkType
#[derive(Debug)]
pub enum ChunkTypeError {
    // Invalid length (furnished length)
    CodeLengthError(usize),
    // Invalid char 
    InvalidChar,
}

impl Display for ChunkTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChunkTypeError::CodeLengthError(len) => writeln!(f, "Invalid code length: {}", len),
            ChunkTypeError::InvalidChar => writeln!(f, "Invalid character encountered!"),
        }
    }
}

impl std::error::Error for ChunkTypeError {}

/// The representation of a chunk type (containing raw bytes of the chunk code)
#[derive(PartialEq, Eq, Debug)]
pub struct ChunkType {
    bytes: [u8; 4],
}

/// Ability to build a ChunkType instance from [u8; 4]
impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;
    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        Ok(Self { bytes: value })
    }
}

/// Ability to build a ChunkType instance from a string slice
impl FromStr for ChunkType {
    type Err = Error;

    /// Transforms a 4-chars string slice into its respective bytes correspondances
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Check for len
        let bytes_s = s.as_bytes();
        if bytes_s.len() != 4 {
            return Err(Box::new(ChunkTypeError::CodeLengthError(bytes_s.len())));
        }

        // Check for invalid chars
        let mut chars = from_utf8(bytes_s).unwrap().chars();
        let valid_chars = chars.all(|c| c.is_ascii_alphabetic());
        if !valid_chars {
            return Err(Box::new(ChunkTypeError::InvalidChar));
        }

        Ok(ChunkType::try_from(<[u8; 4]>::try_from(bytes_s).unwrap())?)
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", from_utf8(&self.bytes).unwrap())
    }
}

impl ChunkType {
    /// Internal raw bytes
    pub fn bytes(&self) -> [u8; 4] {
        self.bytes
    }

    /// Chunk validity check (only ASCII alphabetic bytes and is_reserved_bit_valid)
    fn is_valid(&self) -> bool {
        let bytes = self.bytes(); // Avoid early dropping and temporary variable
        let mut chars = from_utf8(&bytes).unwrap().chars();
        let valid_chars = chars.all(|c| c.is_ascii_alphabetic());
        valid_chars && self.is_reserved_bit_valid()
    }

    /// Is the chunk critical
    fn is_critical(&self) -> bool {
        // First byte
        (self.bytes()[0] & 0x20/*32, which corresponds to hc-lc  bit on or off*/) == 0
    }

    /// Is the chunk public
    fn is_public(&self) -> bool {
        // Second bytes
        (self.bytes()[1] & 0x20) == 0 // Is uppercase
    }

    /// Is the resserved bit on
    fn is_reserved_bit_valid(&self) -> bool {
        // Third byte
        (self.bytes()[2] & 0x20) == 0 // Is uppercase
    }

    /// Is the chunk safe to copy
    fn is_safe_to_copy(&self) -> bool {
        // Fourth byte
        (self.bytes()[3] & 0x20) == 0x20 // Is lowercase
    }
}

mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;


    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116]; // 4 bytes array
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}

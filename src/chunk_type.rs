use std::{str::{self, FromStr, from_utf8}, fmt::Display};

#[derive(PartialEq, Eq)]
pub struct ChunkType {
    code: String,
    bytes: [u8; 4],
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = &'static str;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        Ok(Self{code: from_utf8(&value).map_err(|_| "Couldn't parse to string")?.to_owned(), bytes: value})
    }
}

impl FromStr for ChunkType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ChunkType { code: s.to_owned(), bytes: s.as_bytes().try_into().unwrap() })
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.code)
    }
}

impl ChunkType {
    fn bytes(&self) -> [u8; 4] {
        self.bytes
    }
}
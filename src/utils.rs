use std::fmt::Display;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    //ChunkType
    CodeLengthError(usize), // Invalid length (furnished length)
    InvalidType,            // Invalid code
    InvalidChar, // Invalid char
    //Chunk
    ChunkSliceSizeError, // Tryfrom trait implementation associated error (when checking the fed in slice length)
    PngSliceError, // Tryfrom trait implementation associated error (when checking the fed slice length)
    CRC,           // Wrong CRC at creation (specified is not what the real is)
    //Png
    ChunkNotFound(String), // When searching for a specific chunk with the chunktype, found nothing
    InvalidPngHeader, //When header is not correct when building from byutes slice
    FileError,     // When IO operations fail
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Error::CodeLengthError(sz) => format!("Invalid code length for chunktype encountered (found size: {sz})"),
            Error::ChunkSliceSizeError => "Chunk has invalid sequence of bytes length".to_string(),
            Error::CRC => "Invalid CRC".to_string(),
            Error::ChunkNotFound(typ) => format!("No chunk found for the chunktype {typ}"),
            Error::FileError => "IO error occured".to_string(),
            Error::InvalidType => "Invalid Chunktype detected".to_string(),
            Error::InvalidChar => "Invalid char in type-code".to_string(),
            Error::InvalidPngHeader => "Png contains invalid header".to_string(),
            Error::PngSliceError => "Error while creating a png from a bad bytes sequence".to_string(),
        };
        writeln!(f, "{message}")
    }
}

/// Checked alternative of split_at()
pub fn checked_split_at<T>(slice: &[T], mid: usize) -> Option<(&[T], &[T])> {
    if mid < slice.len() {
        Some(slice.split_at(mid))
    } else {
        None
    }
}

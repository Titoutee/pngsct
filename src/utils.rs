pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    //ChunkType
    CodeLengthError(usize), // Invalid length (furnished length)
    InvalidChar,            // Invalid char
    //Chunk
    ChunkSliceSizeError, // Tryfrom trait implementation associated error (when checking the fed slice length)
    PngSliceError, // Tryfrom trait implementation associated error (when checking the fed slice length)
    CRC,           // Wrong CRC at creation (specified is not what the real is)
    //Png
    ChunkNotFound, // When searching for a specific chunk with the chunktype, found nothing
}

/// Checked alternative of split_at()
pub fn checked_split_at<T>(slice: &[T], mid: usize) -> Option<(&[T], &[T])> {
    if mid < slice.len() {
        Some(slice.split_at(mid))
    } else {
        None
    }
}

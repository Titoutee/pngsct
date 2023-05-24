use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;
use crate::utils::{Error, Result as R};
use std::fs;
use std::path::{PathBuf};
use std::str::FromStr;

//type R<T> = Result<T, Box<dyn std::error::Error>>;

/// Encoding command
pub fn encode(args: EncodeArgs) -> R<()> {
    let contents = contents(args.file_path.clone())?; // Png as bytes
    let mut png = Png::try_from(contents.as_slice())?; // New png with exact same definition as opened one

    let message = args.message;
    let chunk = Chunk::new(ChunkType::from_str(&args.chunk_type)?,message.as_bytes().to_vec());
    png.append_chunk(chunk);
    let new_contents = png.as_bytes();
    let save_path = if let Some(path) = args.output_file {
        path
    } else {
        args.file_path
    }; // If specified, use given path, else overwrite
    fs::write(save_path, new_contents).map_err(|_| Error::FileError)?; // New png file save
    Ok(())
}

pub fn decode(args: DecodeArgs) -> R<()> {
    let png = Png::try_from(contents(args.file_path)?.as_slice())?;
    let c_type = args.chunk_type;
    let r_chunk = png.chunk_by_type(c_type.as_ref()).ok_or(Error::ChunkNotFound(c_type.to_string()))?;
    let conv = r_chunk.data_as_string();
    match conv {
        Ok(message) => {
            println!("Found a message for the specified chunk type: {}", message);
            Ok(())
        }
        _ => Err(Error::InvalidChar)
    }

}

pub fn print(args: PrintArgs) -> R<()> {
    println!("{}", Png::from_file(args.file_path).map_err(|_| Error::FileError)?);
    Ok(())
}

pub fn remove(args: RemoveArgs) -> R<()> {
    let contents = contents(args.file_path.clone())?; // Remove clone call
    let mut png = Png::try_from(contents.as_slice())?;
    png.remove_chunk(&args.chunk_type)?;
    let new_contents = png.as_bytes();
    
    fs::write(args.file_path, new_contents).map_err(|_| Error::FileError)?; // We overwrite in all cases (so the original png file was mutated)
    Ok(())
}

pub fn contents(path: PathBuf) -> R<Vec<u8>> {
    fs::read(path).map_err(|_| Error::FileError)
}
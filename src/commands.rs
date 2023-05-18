use crate::args::{Args, DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;
use crate::utils::Result as R;
/// Encoding command
pub fn encode_command(args: EncodeArgs) -> R<()> {
    let mut png = Png::from_file(args.file_path)?;
    todo!()
}

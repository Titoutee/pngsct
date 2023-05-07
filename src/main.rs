//mod args;
//mod chunk;
mod chunk_type;
mod chunk;
//mod commands;
//mod png
mod utils;

use chunk_type::{ChunkType};
use chunk::Chunk;
use utils::{Error, Result};

fn main() -> Result<()> {
    let arr = [97; 4];
    let inst = ChunkType::try_from(arr).unwrap();
    println!("{}", true.to_string());
    Ok(())
}

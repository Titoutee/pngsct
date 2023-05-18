
mod args;
mod chunk;
mod chunk_type;
mod png;
mod utils;
mod commands;
//use std::fs;
//use png::Png;
use structopt::StructOpt;
//use chunk_type::{ChunkType};
//use structopt::StructOpt;
use utils::{Error, Result};

fn main() -> Result<()> {
    match args::Args::from_args() {
        args::Args::Encode(args) => println!("Encode!"),
        args::Args::Decode(args) => println!("Decode!"),
        args::Args::Remove(args) => println!("Remove!"),
        args::Args::Print(args) => println!("Print!"),
    }
    Ok(())
}

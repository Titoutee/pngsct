mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;
mod utils;

use commands::{decode, encode, print, remove};
use structopt::StructOpt;
use utils::Error;

fn main() -> Result<(), ()> {
    let res = match args::Args::from_args() {
        args::Args::Encode(args) => encode(args),
        args::Args::Decode(args) => decode(args),
        args::Args::Remove(args) => remove(args),
        args::Args::Print(args) => print(args),
    };
    if let Err(my_error) = res {
        println!("{my_error}");
    }
    Ok(())
}

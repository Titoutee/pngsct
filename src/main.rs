
mod args;
mod chunk;
mod chunk_type;
mod png;
mod utils;
mod commands;

use structopt::StructOpt;
use utils::{Error};
use commands::{encode, print, remove, decode};

fn main() -> Result<(), ()> {
    let res = match args::Args::from_args() {
        args::Args::Encode(args) => encode(args),
        args::Args::Decode(args) => decode(args),
        args::Args::Remove(args) => remove(args),
        args::Args::Print(args) => print(args),
    };
    if res.is_err() {
        println!("{:#?}", res);
    }
    Ok(())
}

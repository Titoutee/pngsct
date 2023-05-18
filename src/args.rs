use structopt::StructOpt;
use std::path::PathBuf;

#[derive(Debug, StructOpt)]
pub enum Args {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(Debug, StructOpt)]
pub struct EncodeArgs {
    pub file_path: PathBuf,
    pub chunk_type: String,
    pub message: String,
    pub output_file: Option<PathBuf> // Optional
}

#[derive(Debug, StructOpt)]
pub struct DecodeArgs {
    pub file_path: PathBuf,
    pub chunk_type: String,
}

#[derive(Debug, StructOpt)]
pub struct RemoveArgs {
    pub file_path: PathBuf,
    pub chunk_type: String,
}

#[derive(Debug, StructOpt)]
pub struct PrintArgs {
    pub file_path: PathBuf,
}

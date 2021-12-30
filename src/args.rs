use std::path::PathBuf;

use clap::{Args, Parser};

#[derive(Parser, Debug)]
#[clap(version, author, about)]
pub enum PngArgs {
    /// Encodes a secret message into the given image
    Encode(EncodeArgs),
    /// Decodes a message (prints it) if given a chunk type name
    Decode(DecodeArgs),
    /// Removes chunk from PNG if chunk type exists
    Remove(RemoveArgs),
    /// Prints all chunks in the PNG
    Print(PrintArgs),
}

#[derive(Debug, Args)]
pub struct EncodeArgs {
    /// The path to your image
    pub file_path: PathBuf,
    /// The 4 byte name for your chunk (a-z and A-Z only)
    pub chunk_type: String,
    /// The secret message to store in that chunk
    pub message: String,
}

#[derive(Debug, Args)]
pub struct DecodeArgs {
    /// The path to your image
    pub file_path: PathBuf,
    /// The 4 byte name for your chunk (a-z and A-Z only)
    pub chunk_type: String,
}

#[derive(Debug, Args)]
pub struct RemoveArgs {
    /// The path to your image
    pub file_path: PathBuf,
    /// The 4 byte name for your chunk (a-z and A-Z only)
    pub chunk_type: String,
}

#[derive(Debug, Args)]
pub struct PrintArgs {
    /// The path to your image
    pub file_path: PathBuf,
}

#![allow(dead_code)]

use crate::args::PngArgs;
use clap::StructOpt;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args = PngArgs::parse();

    match args {
        PngArgs::Encode(arg) => commands::encode(arg),
        PngArgs::Decode(arg) => commands::decode(arg),
        PngArgs::Remove(arg) => commands::remove(arg),
        PngArgs::Print(arg) => commands::print(arg),
    }
}

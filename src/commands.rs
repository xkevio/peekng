use std::str::FromStr;

use crate::args::PrintArgs;
use crate::Result;
use crate::{
    args::{DecodeArgs, EncodeArgs, RemoveArgs},
    chunk::Chunk,
    chunk_type::ChunkType,
    png::Png,
};

pub fn encode(args: EncodeArgs) -> Result<()> {
    let bytes = std::fs::read(&args.file_path)?;
    let chunk_type = ChunkType::from_str(&args.chunk_type)?;
    let message: Vec<u8> = args.message.bytes().collect();

    let mut new_png = Png::try_from(&bytes[..])?;
    let chunk = Chunk::new(chunk_type, message);

    new_png.append_chunk(chunk);

    match std::fs::write(&args.file_path, new_png.as_bytes()) {
        Ok(_) => Ok(()),
        Err(e) => Err(Box::new(e)),
    }
}

pub fn decode(args: DecodeArgs) -> Result<()> {
    let bytes = std::fs::read(&args.file_path)?;
    let png = Png::try_from(&bytes[..])?;

    if let Some(chunk) = png.chunk_by_type(&args.chunk_type) {
        let message = chunk.data_as_string()?;
        println!("{}", message);
    } else {
        eprintln!("No chunk with chunk type {} found!", &args.chunk_type);
        return Err("Chunk type not present in the PNG file!".into());
    }

    Ok(())
}

pub fn remove(args: RemoveArgs) -> Result<()> {
    let bytes = std::fs::read(&args.file_path)?;
    let mut png = Png::try_from(&bytes[..])?;
    png.remove_chunk(&args.chunk_type)?;

    match std::fs::write(&args.file_path, png.as_bytes()) {
        Ok(_) => Ok(()),
        Err(e) => Err(Box::new(e)),
    }
}

pub fn print(args: PrintArgs) -> Result<()> {
    let bytes = std::fs::read(&args.file_path)?;
    let png = Png::try_from(&bytes[..])?;

    println!("{}", png);
    Ok(())
}

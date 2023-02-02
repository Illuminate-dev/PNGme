use crate::{args, PngmeError};
use crate::{chunk::Chunk, chunk_type, png::Png};
use std::fs;
use std::str::FromStr;

pub fn encode(args: args::EncodeArgs) -> crate::Result<()> {
    let file = match fs::read(&args.filepath) {
        Ok(x) => x,
        Err(_) => return Err(PngmeError::FileReadError),
    };
    let output_file = args.output_file.unwrap_or(args.filepath);

    let mut png = Png::try_from(file.as_slice())?;
    let chunk_type = chunk_type::ChunkType::from_str(&args.chunk_type)?;
    let chunk = Chunk::new(chunk_type, args.message.as_bytes().to_vec());
    png.append_chunk(chunk);

    match fs::write(output_file, png.as_bytes()) {
        Ok(_) => Ok(()),
        Err(_) => Err(PngmeError::FileWriteError),
    }
}

pub fn decode(args: args::DecodeArgs) -> crate::Result<()> {
    let file = match fs::read(&args.filepath) {
        Ok(x) => x,
        Err(_) => return Err(PngmeError::FileReadError)
    };
    let png = Png::try_from(file.as_slice())?;
    let chunk = match png.chunk_by_type(&args.chunk_type) {
        Some(x) => x,
        None => {
            eprintln!("Couldn't find chunk with name: {}", &args.chunk_type);
            return Err(PngmeError::ChunkTypeInvalid)
        }
    };
    println!("Message: {}", chunk.data_as_string().unwrap());
    Ok(())
}

pub fn remove(args: args::RemoveArgs) {
    let file = fs::read(&args.filepath).unwrap();
    let mut png = Png::try_from(file.as_slice()).unwrap();
    let chunk = png.remove_chunk(&args.chunk_type).unwrap();
    fs::write(&args.filepath, png.as_bytes()).unwrap();
    println!("Removed Chunk: {}", chunk)
}

pub fn print(args: args::PrintArgs) {
    let file = fs::read(&args.filepath).unwrap();
    let png = Png::try_from(file.as_slice()).unwrap();
    println!("{}: {}", args.filepath.display(), png);
}

pub fn run(arg: args::Args) -> crate::Result<()> {
    match arg.command {
        args::Commands::Encode(args) => encode(args),
        args::Commands::Decode(args) => decode(args),
        args::Commands::Remove(args) => remove(args),
        args::Commands::Print(args) => print(args),
    }
}

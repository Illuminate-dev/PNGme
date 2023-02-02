use crate::args;
use crate::{chunk::Chunk, chunk_type, png::Png};
use std::fs;
use std::str::FromStr;

pub fn encode(args: args::EncodeArgs) {
    let file = fs::read(&args.filepath).unwrap();
    let output_file = args.output_file.unwrap_or(args.filepath);

    let mut png = Png::try_from(file.as_slice()).unwrap();
    let chunk_type = chunk_type::ChunkType::from_str(&args.chunk_type).unwrap();
    let chunk = Chunk::new(chunk_type, args.message.as_bytes().to_vec());
    png.append_chunk(chunk);

    fs::write(output_file, png.as_bytes()).unwrap();
}

pub fn decode(args: args::DecodeArgs) {
    let file = fs::read(&args.filepath).unwrap();
    let png = Png::try_from(file.as_slice()).unwrap();
    let chunk = png.chunk_by_type(&args.chunk_type).unwrap();
    println!("Message: {}", chunk.data_as_string().unwrap());
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

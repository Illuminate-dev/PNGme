mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use args::Commands;
use chunk::Chunk;
use clap::Parser;
use png::Png;
use std::fs;
use std::str::FromStr;

fn main() {
    let args = args::Args::parse();

    match args.command {
        Commands::Encode {
            filepath,
            chunk_type,
            message,
            output_file,
        } => {
            let file = fs::read(&filepath).unwrap();
            let output_file = output_file.unwrap_or(filepath);

            let mut png = Png::try_from(file.as_slice()).unwrap();
            let chunk_type = chunk_type::ChunkType::from_str(&chunk_type).unwrap();
            let chunk = Chunk::new(chunk_type, message.as_bytes().to_vec());
            png.append_chunk(chunk);

            fs::write(output_file, png.as_bytes()).unwrap();
        }
        Commands::Decode {
            filepath,
            chunk_type,
        } => {
            let file = fs::read(&filepath).unwrap();
            let png = Png::try_from(file.as_slice()).unwrap();
            let chunk = png.chunk_by_type(&chunk_type).unwrap();
            println!("Message: {}", chunk.data_as_string().unwrap());
        }
        Commands::Print { filepath } => {
            let file = fs::read(&filepath).unwrap();
            let png = Png::try_from(file.as_slice()).unwrap();
            println!("{}: {}", filepath.display(), png);
        }
        Commands::Remove {
            filepath,
            chunk_type,
        } => {
            let file = fs::read(&filepath).unwrap();
            let mut png = Png::try_from(file.as_slice()).unwrap();
            let chunk = png.remove_chunk(&chunk_type).unwrap();
            fs::write(&filepath, png.as_bytes()).unwrap();
            println!("Removed Chunk: {}", chunk)
        }
    }
}

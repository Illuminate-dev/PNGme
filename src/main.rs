mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use args::Commands;
use clap::Parser;

fn main() {
    let args = args::Args::parse();

    match args.command {
        Commands::Encode(args) => commands::encode(args),
        Commands::Decode(args) => commands::decode(args),
        Commands::Print(args) => commands::print(args),
        Commands::Remove(args) => commands::remove(args),
    }
}

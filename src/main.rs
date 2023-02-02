mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use clap::Parser;

fn main() {
    let args = args::Args::parse();

    dbg!(args);
}

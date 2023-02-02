use clap::Parser;
use pngme::Result;
use pngme::commands;
use pngme::args;

fn main() -> Result<()> {
    let args = args::Args::parse();
    commands::run(args)
}

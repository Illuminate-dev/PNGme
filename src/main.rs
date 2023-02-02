use clap::Parser;
use pngme::commands;
use pngme::args;

fn main() {
    let args = args::Args::parse();
    commands::run(args)
}

use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Encode {
        filepath: PathBuf,
        chunk_type: String,
        message: String,
        output_file: Option<PathBuf>,
    },
    Decode {
        filepath: PathBuf,
        chunk_type: String,
    },
    Remove {
        filepath: PathBuf,
        chunk_type: String,
    },
    Print {
        filepath: PathBuf,
    },
}

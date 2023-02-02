use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Encode {
        filepath: String,
        chunk_type: String,
        message: String,
        output_file: Option<String>,
    },
    Decode {
        filepath: String,
        chunk_type: String,
    },
    Remove {
        filepath: String,
        chunk_type: String,
    },
    Print {
        filepath: String,
    },
}

pub mod args;
pub mod chunk;
pub mod chunk_type;
pub mod commands;
pub mod png;

pub type Result<T> = std::result::Result<T, PngmeError>;

#[derive(Debug)]
pub enum PngmeError {
    ChunkTypeInvalid,
    ChunkTypeParsingError,
    PngRemoveError,
    PngInvalidHeader,
    ChunkInvalid,
    ChunkFromUTF8Error,
}
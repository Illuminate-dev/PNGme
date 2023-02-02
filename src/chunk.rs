use std::fmt::Display;

use crate::chunk_type::ChunkType;

type Result<T> = std::result::Result<T, ChunkError>;

#[derive(Debug)]
pub enum ChunkError {
    InvalidChunkError,
    FromUTF8Error,
}

#[derive(Debug)]
pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    chunk_data: Vec<u8>,
    crc: u32,
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:#?}")
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = ChunkError;

    fn try_from(value: &[u8]) -> Result<Self> {
        let length: u32 = u32::from_be_bytes(value[..4].try_into().unwrap());
        let chunk_type: [u8; 4] = value[4..8].try_into().unwrap();
        let chunk_type = ChunkType::try_from(chunk_type).unwrap();
        let end_slice: usize = length.try_into().unwrap();
        let data = value[8..end_slice + 8].to_vec();
        let crc = u32::from_be_bytes(value[value.len() - 4..].try_into().unwrap());
        let real_crc = crc::crc32::checksum_ieee(&value[4..end_slice + 8]);
        if real_crc != crc {
            return Err(ChunkError::InvalidChunkError);
        }
        Ok(Chunk {
            length,
            chunk_type,
            chunk_data: data,
            crc,
        })
    }
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let length: u32 = data.len().try_into().unwrap();

        let mut chunk_and_data = Vec::from(chunk_type.bytes());
        chunk_and_data.extend(data.iter().cloned());

        let crc = crc::crc32::checksum_ieee(&chunk_and_data);
        Chunk {
            length,
            chunk_type,
            chunk_data: data,
            crc,
        }
    }

    fn length(&self) -> u32 {
        self.length
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    fn data(&self) -> &[u8] {
        &self.chunk_data
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn data_as_string(&self) -> Result<String> {
        let data = self.chunk_data.to_vec();
        dbg!(&data);
        match String::from_utf8(data) {
            Ok(x) => Ok(x),
            Err(_) => Err(ChunkError::FromUTF8Error),
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let length = u32_to_u8_array(self.length);
        let chunk_type = self.chunk_type.bytes();
        let crc = u32_to_u8_array(self.crc);

        length
            .iter()
            .copied()
            .chain(chunk_type.iter().copied())
            .chain(self.chunk_data.iter().copied())
            .chain(crc.iter().copied())
            .collect()

    }
}

fn u32_to_u8_array(x: u32) -> [u8; 4] {
    let b1: u8 = (x >> 24) as u8;
    let b2: u8 = (x >> 16) as u8;
    let b3: u8 = (x >> 8) as u8;
    let b4: u8 = (x) as u8;

    [b1, b2, b3, b4]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}

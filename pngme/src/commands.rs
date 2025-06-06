use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;
use anyhow::{bail, Error, Result};
use std::fs;

fn chunk_from_strings(chunk_type: &str, data: &str) -> Result<Chunk> {
    use std::str::FromStr;

    let chunk_type = ChunkType::from_str(chunk_type)?;
    let data: Vec<u8> = data.bytes().collect();

    Ok(Chunk::new(chunk_type, data))
}

pub fn encode(
    file_path: &str,
    chunk_type: &str,
    message: &str,
    output_file: Option<&str>,
) -> Result<(), Error> {
    let contents = fs::read(file_path)?;
    let mut png = Png::try_from(contents)?;
    png.append_chunk(chunk_from_strings(chunk_type, message)?);
    if let Some(output_file) = output_file {
        fs::write(output_file, png.as_bytes())?;
    } else {
        fs::write(file_path, png.as_bytes())?;
    }
    Ok(())
}

pub fn decode(file_path: &str, chunk_type: &str) -> Result<String> {
    let contents = fs::read(file_path)?;
    let png = Png::try_from(contents)?;
    let chunk = png.chunk_by_type(chunk_type);
    if let Some(chunk) = chunk {
        return Ok(chunk.data_as_string()?);
    } else {
        bail!("未找到对应类型的分块")
    }
}

pub fn remove(file_path: &str, chunk_type: &str) -> Result<()> {
    let contents = fs::read(file_path)?;
    let mut png = Png::try_from(contents)?;
    png.remove_first_chunk(chunk_type)?;
    Ok(())
}

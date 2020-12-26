use super::parse_error::ParseError;
use crate::entity::{Chunk, DataChunk, Entity, HeaderAttibute, HeaderChunk, Object, PathObject};
use byteorder::{BigEndian, ByteOrder, ReadBytesExt};
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Cursor, SeekFrom};
use std::time::{Duration, UNIX_EPOCH};

fn read_expect(reader: &mut BufReader<File>, expected: &Vec<u8>) -> Result<(), ParseError> {
    let mut buf: Vec<u8> = vec![0; expected.len()];
    reader.read_exact(&mut buf)?;

    if buf != *expected {
        return Err(ParseError::new("Expected bytes not found"));
    };

    Ok(())
}

fn read_signature(reader: &mut BufReader<File>) -> Result<String, ParseError> {
    read_expect(reader, &vec![0x0d, 'S' as u8, '7' as u8, 0x0d])?;

    let mut version_buf: Vec<u8> = Vec::new();
    reader.read_until(0, &mut version_buf)?;

    let mut version = String::from_utf8(version_buf).unwrap();
    version.pop(); // Remove trailing zero byte

    Ok(version)
}

fn read_chunk(reader: &mut BufReader<File>) -> Result<Chunk, ParseError> {
    let size = reader.read_u32::<BigEndian>()?;

    let mut code_bytes = [0u8; 4];
    reader.read_exact(&mut code_bytes)?;
    let code = String::from_utf8(code_bytes.into()).unwrap();

    let mut data = vec![0u8; (size - 8) as usize];
    reader.read_exact(&mut data)?;

    Ok(Chunk { code, data })
}

fn read_header_attribute(chunk_reader: &mut Cursor<Vec<u8>>) -> Result<HeaderAttibute, ParseError> {
    let size = chunk_reader.read_u32::<BigEndian>()?;

    let mut code_bytes = [0u8; 4];
    chunk_reader.read_exact(&mut code_bytes)?;
    let code = String::from_utf8(code_bytes.into()).unwrap();

    let mut data = vec![0u8; (size - 8) as usize];
    chunk_reader.read_exact(&mut data)?;

    Ok(HeaderAttibute {
        key: code,
        val: data,
    })
}

fn parse_header_chunk(chunk: Chunk) -> Result<HeaderChunk, ParseError> {
    if chunk.code != "HEAD" {
        return Err(ParseError::new("Expected header chunk"));
    }

    let size = chunk.data.len() as u64;
    let mut chunk_reader = Cursor::new(chunk.data);

    let mut other_attributes: Vec<HeaderAttibute> = Vec::new();
    let mut creation_date = None;
    let mut width = 0;
    let mut height = 0;

    while chunk_reader.position() < size {
        let attribute = read_header_attribute(&mut chunk_reader)?;

        match &attribute.key[..] {
            "DATE" => {
                let unix_secs = BigEndian::read_u64(attribute.val.as_slice());
                let duration = Duration::from_secs(unix_secs);
                let date = UNIX_EPOCH + duration;

                creation_date = Some(date);
            }
            "WIDT" => width = BigEndian::read_u16(attribute.val.as_slice()),
            "HEIG" => height = BigEndian::read_u16(attribute.val.as_slice()),
            _ => other_attributes.push(attribute),
        }
    }

    Ok(HeaderChunk {
        creation_date,
        other_attributes,
        width,
        height,
    })
}

fn parse_data_chunk(chunk: Chunk) -> Result<DataChunk, ParseError> {
    if chunk.code != "DATA" {
        return Err(ParseError::new("Expected data chunk"));
    }

    let total_size = chunk.data.len() as u64;
    let mut chunk_reader = Cursor::new(chunk.data);

    let mut objects = Vec::new();

    while chunk_reader.position() < total_size {
        let start_pos = chunk_reader.position();
        let size = chunk_reader.read_u32::<BigEndian>()?;
        let obj_type = chunk_reader.read_u8()? as char;

        let object = match obj_type {
            'P' => {
                let mut color = [0; 3];
                chunk_reader.read_exact(&mut color)?;

                let mut points: Vec<[u16; 2]> = Vec::new();
                while chunk_reader.position() < start_pos + size as u64 {
                    let x = chunk_reader.read_u16::<BigEndian>()?;
                    let y = chunk_reader.read_u16::<BigEndian>()?;

                    points.push([x, y]);
                }

                Object::Path(PathObject { color, points })
            }
            _ => {
                return Err(ParseError::new(&format!(
                    "Unexpected object type with code '{}'",
                    obj_type
                )))
            }
        };

        objects.push(object);
    }

    Ok(DataChunk { objects })
}

pub fn read(path: &str) -> Result<Entity, ParseError> {
    let file = File::open(path)?;
    let file_size = file.metadata().unwrap().len();
    let mut reader = BufReader::new(file);

    let version = read_signature(&mut reader)?;

    let header_chunk = parse_header_chunk(read_chunk(&mut reader)?)?;

    let mut other_chunks = Vec::new();
    let mut data_chunks = Vec::new();

    while reader.seek(SeekFrom::Current(0)).unwrap() < file_size {
        let chunk = read_chunk(&mut reader)?;

        match &chunk.code[..] {
            "DATA" => data_chunks.push(parse_data_chunk(chunk)?),
            _ => other_chunks.push(chunk),
        }
    }

    Ok(Entity {
        header_chunk,
        version,
        data_chunks,
        other_chunks,
    })
}

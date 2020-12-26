use crate::entity::{DataChunk, Entity, HeaderAttibute, HeaderChunk, Object};
use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::time::UNIX_EPOCH;

fn write_chunk(file: &mut File, code: &str, data: Vec<u8>) {
    // Chunk size
    file.write_all(&(data.len() as u32 + 8).to_be_bytes())
        .unwrap();

    // Chunk code
    file.write_all(code[..4].as_bytes()).unwrap();

    // Data
    file.write_all(data.as_slice()).unwrap();
}

fn write_header_chunk(file: &mut File, header: HeaderChunk) {
    let mut all_attributes = vec![HeaderAttibute {
        key: "SIZE".to_owned(),
        val: 0u64.to_be_bytes().to_vec(),
    }];

    all_attributes.extend_from_slice(header.other_attributes.as_slice());

    match header.creation_date {
        None => (),
        Some(date) => all_attributes.push(HeaderAttibute {
            key: "DATE".to_owned(),
            val: date
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                .to_be_bytes()
                .to_vec(),
        }),
    }

    let mut data: Vec<u8> = Vec::new();

    for attr in all_attributes {
        // Size
        let attr_size = 8 + attr.val.len() as u32;
        data.extend_from_slice(&attr_size.to_be_bytes()[..]);

        // Key
        data.extend_from_slice(attr.key[..4].as_bytes());

        // Value
        data.extend_from_slice(&attr.val[..]);
    }

    write_chunk(file, "HEAD", data);
}

fn write_data_chunk(file: &mut File, chunk: DataChunk) {
    let mut data: Vec<u8> = Vec::new();

    for obj in chunk.objects {
        match obj {
            Object::Path(path) => {
                // Size
                let size = 8 + path.points.len() as u32 * 4;
                data.extend_from_slice(&size.to_be_bytes());

                // Type
                data.push('P' as u8);

                // Color
                data.extend_from_slice(&path.color);

                // Points
                for [x, y] in path.points {
                    data.extend_from_slice(&[x.to_be_bytes(), y.to_be_bytes()].concat());
                }
            }
        }
    }

    write_chunk(file, "DATA", data);
}

pub fn write(out_path: &str, entity: Entity) {
    let mut file = File::create(out_path).unwrap();

    // Magic number
    let magic_bytes = &[&[0x0d], "S7".as_bytes(), &[0x0d]].concat();
    file.write_all(magic_bytes).unwrap();

    // Version
    file.write_all(entity.version.as_bytes()).unwrap();
    file.write_all(&[0]).unwrap();

    write_header_chunk(&mut file, entity.header_chunk);

    for chunk in entity.data_chunks {
        write_data_chunk(&mut file, chunk);
    }

    write_chunk(&mut file, "FEND", Vec::new());

    // Overwrite file size
    let file_size = file.metadata().unwrap().len();
    file.seek(SeekFrom::Start(26)).unwrap();
    file.write_all(&file_size.to_be_bytes()[..]).unwrap();

    log::debug!("Output file has size of {} bytes", file_size);
}

use super::image::{Coords, Image};
use std::fs::File;
use std::io::{
    prelude::{Seek, Write},
    SeekFrom,
};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn read_png(path: &str) -> Image {
    let in_file = File::open(path).unwrap();

    let mut decoder = png::Decoder::new(in_file);
    decoder.set_transformations(png::Transformations::EXPAND);

    let (info, mut reader) = decoder.read_info().unwrap();

    let mut buf = vec![0; info.buffer_size()];
    reader.next_frame(&mut buf).unwrap();

    let pixels = buf
        .chunks(4)
        .map(|x| (x[0], x[1], x[2], x[3], false))
        .collect();

    return Image::new(buf, pixels, info.width, info.height);
}

pub struct S4HeadAttribute {
    key: [u8; 4],
    val: Vec<u8>,
}

enum S4PathType {
    Path,
}

impl From<S4PathType> for u8 {
    fn from(path_type: S4PathType) -> Self {
        match path_type {
            S4PathType::Path => 0x50, // 'P'
        }
    }
}

pub fn write_s4_section(file: &mut File, code: &str, data: &[u8]) {
    // Chunk size
    file.write_all(&(data.len() as u32 + 8).to_be_bytes())
        .unwrap();

    // Chunk code
    file.write_all(code[..4].as_bytes()).unwrap();

    // Data
    file.write_all(data).unwrap();
}

fn write_s4_head(file: &mut File, attributes: Vec<S4HeadAttribute>) {
    let mut data: Vec<u8> = vec![];

    for attribute in attributes {
        // Size
        let attr_size = 8 + attribute.val.len() as u32;
        data.extend_from_slice(&attr_size.to_be_bytes()[..]);

        // Key
        data.extend_from_slice(&attribute.key[..]);

        // Value
        data.extend_from_slice(&attribute.val[..]);
    }

    write_s4_section(file, "HEAD", &data[..]);
}

fn write_s4_data(file: &mut File, paths: Vec<Vec<Coords>>) {
    let mut data: Vec<u8> = vec![];

    for path in paths {
        // Size
        let size = 8 + path.len() as u32 * 4;
        data.extend_from_slice(&size.to_be_bytes());

        // Type
        data.push(S4PathType::Path.into());

        // Color
        data.extend_from_slice(&[255, 0, 0, 255]);

        // Points
        for (x, y) in path {
            data.extend_from_slice(&[(x as u16).to_be_bytes(), (y as u16).to_be_bytes()].concat());
        }
    }

    write_s4_section(file, "DATA", &data[..]);
}

pub fn write_s4(out_path: &str, image_paths: Vec<Vec<Coords>>) {
    let mut file = File::create(out_path).unwrap();

    // Magic number
    let magic_bytes = &[&[0x0d], "S4".as_bytes(), &[0x0d]].concat();
    file.write_all(magic_bytes).unwrap();

    // Version
    file.write_all(b"1.0.0\0").unwrap();

    // HEAD
    let attributes = vec![
        S4HeadAttribute {
            key: b"FLEN".to_owned(),
            val: 0u64.to_be_bytes().to_vec(), // Uses a fake file size that we will overwrite at the end
        },
        S4HeadAttribute {
            key: b"DATE".to_owned(),
            val: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                .to_be_bytes()
                .to_vec(),
        },
    ];
    write_s4_head(&mut file, attributes);

    // DATA
    write_s4_data(&mut file, image_paths);

    // FEND
    write_s4_section(&mut file, "FEND", &[]);

    // Overwrite file size
    file.seek(SeekFrom::Start(26)).unwrap();
    file.write_all(&file.metadata().unwrap().len().to_be_bytes()[..])
        .unwrap();
}

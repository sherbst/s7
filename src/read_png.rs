use super::image::Image;
use std::fs::File;

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

use super::s7_image::Image;
use image::io::Reader as ImageReader;
use image::DynamicImage;

pub fn read_png(path: &str) -> Image {
    let image = ImageReader::open(path).unwrap().decode().unwrap();

    match image {
        DynamicImage::ImageRgb8(rgb_image) => Image::new(rgb_image),
        _ => panic!("Only accepts RGB images for now"),
    }
}

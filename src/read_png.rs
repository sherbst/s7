use super::s7_image::Image;
use image::io::Reader as ImageReader;
use image::{DynamicImage, ImageBuffer, Rgb, Rgba};
use imageproc::map::map_colors;

fn strip_alpha(rgba_image: ImageBuffer<Rgba<u8>, Vec<u8>>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    map_colors(&rgba_image, |rgba| Rgb([rgba.0[0], rgba.0[1], rgba.0[2]]))
}

pub fn read_png(path: &str) -> Image {
    let image = ImageReader::open(path).unwrap().decode().unwrap();

    match image {
        DynamicImage::ImageRgb8(rgb_image) => Image::new(rgb_image),
        DynamicImage::ImageRgba8(rgba_image) => Image::new(strip_alpha(rgba_image)),
        _ => panic!("Only accepts RGB and RGBA images for now"),
    }
}

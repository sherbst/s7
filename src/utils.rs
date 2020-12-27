use image::RgbImage;
use imageproc::point::Point;

pub fn blank_image(width: u16, height: u16) -> RgbImage {
    RgbImage::from_raw(
        width as u32,
        height as u32,
        vec![255; width as usize * height as usize * 3],
    )
    .unwrap()
}

pub fn convert_points(points: Vec<[u16; 2]>) -> Vec<Point<i32>> {
    points
        .iter()
        .map(|[x, y]| Point::new(*x as i32, *y as i32))
        .collect()
}

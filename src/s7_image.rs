use image::RgbImage;
use std::convert::{From, Into};

pub type Coords = (u16, u16);

const RGB_DIFFERENCE: u8 = 3;

#[derive(Debug)]
pub struct Rgb(pub u8, pub u8, pub u8);

impl From<Rgb> for image::Rgb<u8> {
    fn from(rgb: Rgb) -> Self {
        Self([rgb.0, rgb.1, rgb.2])
    }
}

impl From<[u8; 3]> for Rgb {
    fn from(color: [u8; 3]) -> Self {
        let [r, g, b] = color;
        Self(r, g, b)
    }
}

impl From<Rgb> for [u8; 3] {
    fn from(rgb: Rgb) -> Self {
        [rgb.0, rgb.1, rgb.2]
    }
}

#[derive(Clone)]
pub struct Image {
    pub width: u16,
    pub height: u16,
    buf: RgbImage,
    checked_pixels: Vec<bool>,
}

impl Image {
    pub fn new(image: RgbImage) -> Self {
        return Self {
            width: image.width() as u16,
            height: image.height() as u16,
            buf: image,
            checked_pixels: vec![false; (image.width() * image.height()) as usize],
        };
    }

    fn get_pixel_index(&self, coords: Coords) -> usize {
        let (x, y) = coords;
        x as usize + y as usize * self.width as usize
    }

    pub fn get_pixel(&self, coords: Coords) -> Rgb {
        let (x, y) = coords;
        let [r, g, b] = self.buf.get_pixel(x as u32, y as u32).0;
        Rgb(r, g, b)
    }

    pub fn is_valid_coords(&self, coords: Coords) -> bool {
        let (x, y) = coords;
        x < self.width && y < self.height
    }

    pub fn pixel_is_checked(&self, coords: Coords) -> bool {
        self.checked_pixels[self.get_pixel_index(coords)]
    }

    pub fn set_pixel_is_checked(&mut self, coords: Coords, is_checked: bool) {
        let pix = self
            .checked_pixels
            .get_mut(self.get_pixel_index(coords))
            .unwrap();

        *pix = is_checked;
    }

    pub fn compare_pixels(&self, a_coords: Coords, b_coords: Coords) -> bool {
        let a = self.get_pixel(a_coords);
        let b = self.get_pixel(b_coords);

        let a_avg = (a.0 as u16 + a.1 as u16 + a.2 as u16) / 3;
        let b_avg = (b.0 as u16 + b.1 as u16 + b.2 as u16) / 3;

        let diff = (a_avg as i16 - b_avg as i16).abs() as u8;
        diff <= RGB_DIFFERENCE
    }
}

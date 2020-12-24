pub type Coords = (u32, u32);
pub type Pixel = (u8, u8, u8, u8, bool);

#[derive(Clone)]
pub struct Image {
    bytes: Vec<u8>,
    pixels: Vec<Pixel>,
    pub width: u32,
    pub height: u32,
}

impl Image {
    pub fn new(bytes: Vec<u8>, pixels: Vec<Pixel>, width: u32, height: u32) -> Self {
        return Self {
            bytes,
            pixels,
            width,
            height,
        };
    }

    pub fn get_pixel_index(&self, coords: Coords) -> usize {
        let (x, y) = coords;
        (x + y * self.width) as usize
    }

    pub fn get_pixel(&self, coords: Coords) -> Pixel {
        self.pixels[self.get_pixel_index(coords)]
    }

    pub fn is_valid_coords(&self, coords: Coords) -> bool {
        let (x, y) = coords;
        x < self.width && y < self.height
    }

    pub fn pixel_is_checked(&self, coords: Coords) -> bool {
        let pixel = self.get_pixel(coords);

        pixel.4
    }

    pub fn set_pixel_is_checked(&mut self, coords: Coords, is_checked: bool) {
        let index = self.get_pixel_index(coords);
        let pixel = self.pixels.get_mut(index).unwrap();

        pixel.4 = is_checked;
    }
}

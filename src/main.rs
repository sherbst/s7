use std::fs::File;
use std::ops::Range;

type Coords = (u32, u32);
type Pixel = (u8, u8, u8, u8, bool);

pub struct Image {
    pixels: Vec<Pixel>,
    pub width: u32,
    pub height: u32,
}

impl Image {
    fn new(pixels: Vec<Pixel>, width: u32, height: u32) -> Self {
        return Self {
            pixels,
            width,
            height,
        };
    }

    fn get_pixel(&self, coords: Coords) -> Pixel {
        let (x, y) = coords;
        self.pixels[(x + y * self.height) as usize]
    }

    fn is_valid_coords(&self, coords: Coords) -> bool {
        let (x, y) = coords;
        x < self.width && y < self.height
    }

    fn pixel_is_checked(&self, coords: Coords) -> bool {
        let pixel = self.get_pixel(coords);
        pixel.4
    }

    fn set_pixel_is_checked(&mut self, coords: Coords, is_checked: bool) {
        let (x, y) = coords;
        let pixel = self.pixels
            .get_mut((x + y * self.height) as usize)
            .unwrap();

        pixel.4 = is_checked;
    }
}

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

    return Image::new(pixels, info.width, info.height);
}

fn compare_pixels(image: &Image, a_coords: Coords, b_coords: Coords) -> bool {
    let a = image.get_pixel(a_coords);
    let b = image.get_pixel(b_coords);

    a == b
}

fn has_unique_neighbors(image: &Image, coords: Coords) -> bool {
    let (pix_x, pix_y) = coords;

    for x in -1..2 {
        for y in -1..2 {
            if x == 0 && y == 0 {
                continue;
            }

            let neighbor_x = pix_x as i32 + x;
            let neighbor_y = pix_y as i32 + y;

            if neighbor_x < 0 || neighbor_y < 0 {
                continue;
            }

            let neighbor_coords = (neighbor_x as u32, neighbor_y as u32);

            if !compare_pixels(image, neighbor_coords, coords) {
                return true;
            }
        }
    }

    false
}

fn is_edge_pixel(image: &Image, coords: Coords) -> bool {
    let (x, y) = coords;

    if x == 0 || x == image.width - 1 || y == 0 || y == image.height - 1 {
        true
    } else if has_unique_neighbors(image, coords) {
        true
    } else {
        false
    }
}

fn get_next_pixel_coords(image: &Image, coords: Coords, path: &Vec<Coords>) -> Option<Coords> {
    let (pix_x, pix_y) = coords;

    for x in -1..2 {
        for y in -1..2 {
            if x == 0 && y == 0 {
                continue;
            }

            let neighbor_x = pix_x as i32 + x;
            let neighbor_y = pix_y as i32 + y;

            if neighbor_x < 0 || neighbor_y < 0 {
                continue;
            }

            let neighbor_coords = (neighbor_x as u32, neighbor_y as u32);

            if !image.is_valid_coords(neighbor_coords) {
                continue;
            }

            if path.contains(&neighbor_coords) {
                continue;
            }

            if !is_edge_pixel(image, neighbor_coords) {
                continue;
            }

            if compare_pixels(image, coords, neighbor_coords) {
                return Some(neighbor_coords);
            }
        }
    }

    None
}

fn get_edge_path(image: &mut Image, start_coords: Coords) -> Vec<Coords> {
    let mut pixel_coords = start_coords;
    let mut path = vec![start_coords];

    loop {
        pixel_coords = match get_next_pixel_coords(image, pixel_coords, &path) {
            Some(coords) => coords,
            None => return path,
        };

        path.push(pixel_coords);
        image.set_pixel_is_checked(pixel_coords, true);
    }
}

pub fn get_edge_paths(image: &mut Image, x_range: Range<u32>, y_range: Range<u32>) -> Vec<Vec<Coords>> {
    let mut paths: Vec<Vec<Coords>> = vec![];

    for y in y_range {
        for x in x_range.clone() {
            let coords = (x, y);

            if image.pixel_is_checked(coords) {
                continue;
            }

            image.set_pixel_is_checked(coords, true);

            if is_edge_pixel(image, coords) {
                let path = get_edge_path(image, coords);
                paths.push(path.clone());
            }
        }

        if y == 100 {
            break;
        }
    }

    paths
}

fn main() {
    let mut image = read_png("input/input.png");

    let width = image.width;
    let height = image.height;

    println!("Finding paths...");

    println!(
        "{}",
        get_edge_paths(&mut image, 0..width, 0..height).len()
    )
}

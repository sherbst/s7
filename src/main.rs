use std::fs::File;
use std::ops::Range;

type Coords = (u32, u32);

pub struct Image {
    bytes: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

impl Image {
    fn new(bytes: Vec<u8>, width: u32, height: u32) -> Self {
        return Self {
            bytes,
            width,
            height,
        };
    }

    fn get_pixel(&self, coords: Coords) -> [u8; 4] {
        let (x, y) = coords;
        let base_index = ((self.width * y * 4) + (x * 4)) as usize;

        [
            self.bytes[base_index],
            self.bytes[base_index + 1],
            self.bytes[base_index + 2],
            self.bytes[base_index + 3],
        ]
    }

    fn is_valid_coords(&self, coords: Coords) -> bool {
        let (x, y) = coords;
        x < self.width && y < self.height
    }
}

pub fn read_png(path: &str) -> Image {
    let in_file = File::open(path).unwrap();

    let mut decoder = png::Decoder::new(in_file);
    decoder.set_transformations(png::Transformations::EXPAND);

    let (info, mut reader) = decoder.read_info().unwrap();

    let mut buf = vec![0; info.buffer_size()];
    reader.next_frame(&mut buf).unwrap();

    return Image::new(buf, info.width, info.height);
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

fn get_edge_path(image: &Image, start_coords: Coords) -> Vec<Coords> {
    let mut pixel_coords = start_coords;
    let mut path = vec![start_coords];

    loop {
        pixel_coords = match get_next_pixel_coords(image, pixel_coords, &path) {
            Some(coords) => coords,
            None => return path,
        };

        path.push(pixel_coords);
    }
}

pub fn get_edge_paths(image: &Image, x_range: Range<u32>, y_range: Range<u32>) -> Vec<Vec<Coords>> {
    let mut checked_pixels: Vec<Coords> = vec![];
    let mut paths: Vec<Vec<Coords>> = vec![];

    for y in y_range {
        println!("Scanning row {}", y);

        for x in x_range.clone() {
            let coords = (x, y);

            if checked_pixels.contains(&coords) {
                continue;
            }

            checked_pixels.push(coords);

            if is_edge_pixel(image, coords) {
                let mut path = get_edge_path(image, coords);
                paths.push(path.clone());

                checked_pixels.append(&mut path);
            }
        }

        if y == 30 {
            break;
        }
    }

    paths
}

fn main() {
    let image = read_png("input.png");

    println!("Finding paths...");

    println!(
        "{}",
        get_edge_paths(&image, 0..image.width, 0..image.height).len()
    )
}

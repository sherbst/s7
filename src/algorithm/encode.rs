use crate::entity::{DataChunk, Entity, HeaderChunk, Object, PathObject};
use crate::image::{Coords, Image};
use std::ops::Range;
use std::time::SystemTime;

const RGB_DIFFERENCE: u8 = 3;

fn compare_pixels(image: &Image, a_coords: Coords, b_coords: Coords) -> bool {
    let a = image.get_pixel(a_coords);
    let b = image.get_pixel(b_coords);

    let a_avg = (a.0 as u16 + a.1 as u16 + a.2 as u16) / 3;
    let b_avg = (b.0 as u16 + b.1 as u16 + b.2 as u16) / 3;

    let diff = (a_avg as i16 - b_avg as i16).abs() as u8;
    diff <= RGB_DIFFERENCE
}

fn has_unique_neighbors(image: &Image, coords: Coords) -> bool {
    let [pix_x, pix_y] = coords;

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

            let neighbor_coords = [neighbor_x as u16, neighbor_y as u16];

            if !compare_pixels(image, neighbor_coords, coords) {
                return true;
            }
        }
    }

    false
}

fn is_edge_pixel(image: &Image, coords: Coords) -> bool {
    let [x, y] = coords;

    if x == 0 || x == image.width - 1 || y == 0 || y == image.height - 1 {
        true
    } else if has_unique_neighbors(image, coords) {
        true
    } else {
        false
    }
}

fn get_next_pixel_coords(image: &Image, coords: Coords) -> Option<Coords> {
    let [pix_x, pix_y] = coords;

    for y in -1..2 {
        for x in -1..2 {
            if x == 0 && y == 0 {
                continue;
            }

            let neighbor_x = pix_x as i32 + x;
            let neighbor_y = pix_y as i32 + y;

            if neighbor_x < 0 || neighbor_y < 0 {
                continue;
            }

            let neighbor_coords = [neighbor_x as u16, neighbor_y as u16];

            if !image.is_valid_coords(neighbor_coords) {
                continue;
            }

            if image.pixel_is_checked(neighbor_coords) {
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

fn get_object(image: &mut Image, start_coords: Coords) -> (Object, (Coords, Coords)) {
    let mut min_x = 0u16;
    let mut min_y = 0u16;
    let mut max_x = 0u16;
    let mut max_y = 0u16;

    let mut last_coords = start_coords;
    let mut cur_coords = start_coords;

    let mut points = vec![start_coords];

    loop {
        let next_coords = match get_next_pixel_coords(image, cur_coords) {
            Some(coords) => coords,
            None => {
                points.push(cur_coords);

                min_x = min_x.min(cur_coords[0]);
                min_y = min_y.min(cur_coords[1]);
                max_x = max_x.max(cur_coords[0]);
                max_y = max_y.max(cur_coords[1]);

                let bounds = ([min_x, max_y], [max_x, min_y]);

                return (
                    Object::Path(PathObject {
                        color: [255, 0, 0],
                        points,
                    }),
                    bounds,
                );
            }
        };

        image.set_pixel_is_checked(next_coords, true);

        if !(cur_coords[0] == last_coords[0] && cur_coords[0] == next_coords[0])
            && !(cur_coords[1] == last_coords[1] && cur_coords[1] == next_coords[1])
        {
            points.push(cur_coords);

            min_x = min_x.min(cur_coords[0]);
            min_y = min_y.min(cur_coords[1]);
            max_x = max_x.max(cur_coords[0]);
            max_y = max_y.max(cur_coords[1]);
        }

        last_coords = cur_coords;
        cur_coords = next_coords;
    }
}

fn get_objects(
    image: &mut Image,
    x_range: Range<u16>,
    y_range: Range<u16>,
    ignore_color_pix: Option<Coords>,
) -> Vec<Object> {
    let mut objects: Vec<Object> = Vec::new();

    for y in y_range {
        for x in x_range.clone() {
            let coords = [x, y];

            if image.pixel_is_checked(coords) {
                continue;
            }

            image.set_pixel_is_checked(coords, true);

            if is_edge_pixel(image, coords) {
                match ignore_color_pix {
                    Some(cmp_coords) => {
                        if compare_pixels(image, coords, cmp_coords) {
                            continue;
                        }
                    }
                    None => (),
                }

                let (object, bounds) = get_object(image, coords);
                objects.push(object);

                let ([min_x, max_y], [max_x, min_y]) = bounds;
                let mut interior_paths =
                    get_objects(image, min_x..max_x, min_y..max_y, Some(coords));
                objects.append(&mut interior_paths);
            }
        }
    }

    objects
}

pub fn encode(mut image: Image) -> Entity {
    let width = image.width.clone();
    let height = image.height.clone();

    let objects = get_objects(&mut image, 0..width, 0..height, None);

    let data_chunk = DataChunk { objects };

    let header_chunk = HeaderChunk {
        creation_date: Some(SystemTime::now()),
        other_attributes: Vec::new(),
    };

    Entity {
        version: "1.0.0".to_owned(),
        data_chunks: vec![data_chunk],
        header_chunk,
        other_chunks: Vec::new(),
    }
}

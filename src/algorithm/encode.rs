use crate::entity::{DataChunk, Entity, HeaderChunk, Object, PathObject};
use crate::s7_image::{Coords, Image};
use std::ops::Range;
use std::time::SystemTime;

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

            let neighbor_coords = (neighbor_x as u16, neighbor_y as u16);

            if !image.compare_pixels(neighbor_coords, coords) {
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

fn get_next_pixel_coords(image: &Image, coords: Coords) -> Option<Coords> {
    let (pix_x, pix_y) = coords;

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

            let neighbor_coords = (neighbor_x as u16, neighbor_y as u16);

            if !image.is_valid_coords(neighbor_coords) {
                continue;
            }

            if image.pixel_is_checked(neighbor_coords) {
                continue;
            }

            if !is_edge_pixel(image, neighbor_coords) {
                continue;
            }

            if image.compare_pixels(coords, neighbor_coords) {
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

                min_x = min_x.min(cur_coords.0);
                min_y = min_y.min(cur_coords.1);
                max_x = max_x.max(cur_coords.0);
                max_y = max_y.max(cur_coords.1);

                let bounds = ((min_x, max_y), (max_x, min_y));
                let start_pixel = image.get_pixel(start_coords);

                return (
                    Object::Path(PathObject {
                        color: start_pixel,
                        points,
                    }),
                    bounds,
                );
            }
        };

        image.set_pixel_is_checked(next_coords, true);

        if !(cur_coords.0 == last_coords.0 && cur_coords.0 == next_coords.0)
            && !(cur_coords.1 == last_coords.1 && cur_coords.1 == next_coords.1)
        {
            points.push(cur_coords);

            min_x = min_x.min(cur_coords.0);
            min_y = min_y.min(cur_coords.1);
            max_x = max_x.max(cur_coords.0);
            max_y = max_y.max(cur_coords.1);
        }

        last_coords = cur_coords;
        cur_coords = next_coords;
    }
}

fn get_objects(image: &mut Image, x_range: Range<u16>, y_range: Range<u16>) -> Vec<Object> {
    let mut objects: Vec<Object> = Vec::new();

    for y in y_range {
        for x in x_range.clone() {
            let coords = (x, y);

            if image.pixel_is_checked(coords) {
                continue;
            }

            image.set_pixel_is_checked(coords, true);

            if is_edge_pixel(image, coords) {
                let (object, bounds) = get_object(image, coords);
                objects.push(object);

                let ((min_x, max_y), (max_x, min_y)) = bounds;
                let mut interior_paths = get_objects(image, min_x..max_x, min_y..max_y);
                objects.append(&mut interior_paths);
            }
        }
    }

    objects
}

pub fn encode(mut image: Image) -> Entity {
    let width = image.width.clone();
    let height = image.height.clone();

    let objects = get_objects(&mut image, 0..width, 0..height);

    let data_chunk = DataChunk { objects };

    let header_chunk = HeaderChunk {
        creation_date: Some(SystemTime::now()),
        other_attributes: Vec::new(),
        width,
        height,
    };

    Entity {
        version: "1.0.0".to_owned(),
        data_chunks: vec![data_chunk],
        header_chunk,
        other_chunks: Vec::new(),
    }
}

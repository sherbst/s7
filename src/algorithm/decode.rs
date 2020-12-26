use crate::entity::{Entity, Object};
use image::{Rgb, RgbImage};
use imageproc::drawing::{draw_hollow_rect_mut, draw_polygon_mut};
use imageproc::point::Point;
use imageproc::rect::Rect;

pub fn decode(entity: Entity) -> RgbImage {
    let header_chunk = entity.header_chunk;

    let width = header_chunk.width;
    let height = header_chunk.height;

    let mut image = RgbImage::from_raw(
        width.into(),
        height.into(),
        vec![255; width as usize * height as usize * 3],
    )
    .unwrap();

    for data_chunk in entity.data_chunks {
        for object in data_chunk.objects {
            match object {
                Object::Path(path) => {
                    let mut points: Vec<Point<i32>> = path
                        .points
                        .iter()
                        .map(|[x, y]| Point::new(*x as i32, *y as i32))
                        .collect();

                    if points[0] == points[points.len() - 1] {
                        points = points.iter().skip(1).map(|x| *x).collect();
                    }

                    if points.len() == 1 {
                        let pt = points[0];
                        draw_hollow_rect_mut(
                            &mut image,
                            Rect::at(pt.x, pt.y).of_size(1, 1),
                            Rgb(path.color),
                        );
                    } else {
                        draw_polygon_mut(&mut image, &points, Rgb(path.color));
                    }
                }
            }
        }
    }

    image
}

use crate::entity::{Entity, Object};
use crate::utils::blank_image;
use crate::utils::convert_points;
use image::{Rgb, RgbImage};
use imageproc::drawing::{draw_hollow_rect_mut, draw_polygon_mut};
use imageproc::rect::Rect;

pub fn decode(entity: Entity) -> RgbImage {
    let header_chunk = entity.header_chunk;

    let mut image = blank_image(header_chunk.width, header_chunk.height);

    for data_chunk in entity.data_chunks {
        for object in data_chunk.objects {
            match object {
                Object::Path(path) => {
                    let mut points = convert_points(path.points);
                    if points[0] == points[points.len() - 1] {
                        points = points.iter().skip(1).map(|x| *x).collect();
                    }

                    if points.len() == 1 {
                        let pt = points[0];
                        draw_hollow_rect_mut(
                            &mut image,
                            Rect::at(pt.x, pt.y).of_size(1, 1),
                            path.color.into(),
                        );
                    } else {
                        draw_polygon_mut(&mut image, &points, path.color.into());
                    }
                }
            }
        }
    }

    image
}

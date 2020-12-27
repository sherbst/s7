use crate::cli_error::CliError;
use clap::{App, Arg, ArgMatches, SubCommand};
use image::{Rgb, RgbImage};
use imageproc::drawing as draw;
use s7::entity::{Entity, Object};
use s7::serialization::read::read;
use s7::utils::blank_image;
use std::convert::From;

enum ColorType {
    Original,
    Alternating,
}

impl From<&str> for ColorType {
    fn from(value: &str) -> Self {
        match value {
            "original" => Self::Original,
            "alternating" => Self::Alternating,
            _ => panic!("Unexpected value {}", value),
        }
    }
}

pub fn definition<'a>() -> App<'a, 'a> {
    SubCommand::with_name("debug")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the path of the input file")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("FILE")
                .help("Sets the path of the output file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("outlines")
                .short("O")
                .long("outlines")
                .help("Draws the outlines of each path on a white background"),
        )
        .arg(
            Arg::with_name("outline-color")
                .long("outline-color")
                .help("Sets the method used for picking the colors of the outlines")
                .takes_value(true)
                .value_name("COLOR")
                .possible_values(&["original", "alternating"])
                .default_value("original"),
        )
}

fn draw_outlines(image: &mut RgbImage, entity: &Entity, color_type: ColorType) {
    for data in &entity.data_chunks {
        for (index, object) in &data
            .objects
            .iter()
            .enumerate()
            .collect::<Vec<(usize, &Object)>>()
        {
            let color = match color_type {
                ColorType::Alternating => {
                    if index % 4 == 0 {
                        Rgb([255, 0, 0])
                    } else if (index + 1) % 4 == 0 {
                        Rgb([0, 255, 0])
                    } else if (index + 2) % 4 == 0 {
                        Rgb([0, 0, 255])
                    } else {
                        Rgb([0, 0, 0])
                    }
                }
                ColorType::Original => match object {
                    Object::Path(path) => Rgb(path.color),
                },
            };

            match object {
                Object::Path(path) => {
                    let a = path.points.iter().skip(1);
                    let b = path.points.iter().take(path.points.len() - 1);

                    let lines: Vec<(&[u16; 2], &[u16; 2])> = a.zip(b).collect();

                    for (a, b) in lines {
                        let [ax, ay] = a;
                        let [bx, by] = b;

                        draw::draw_line_segment_mut(
                            image,
                            (*ax as f32, *ay as f32),
                            (*bx as f32, *by as f32),
                            color,
                        )
                    }
                }
            }
        }
    }
}

pub fn exec(matches: &ArgMatches) -> Result<(), CliError> {
    let input_path = matches.value_of("INPUT").unwrap();
    let entity = read(input_path).unwrap();
    let mut image = blank_image(entity.header_chunk.width, entity.header_chunk.height);

    if matches.is_present("outlines") {
        draw_outlines(
            &mut image,
            &entity,
            matches.value_of("outline-color").unwrap().into(),
        );
    }

    match matches.value_of("output") {
        None => (),
        Some(path) => image.save(path).unwrap(),
    }

    Ok(())
}

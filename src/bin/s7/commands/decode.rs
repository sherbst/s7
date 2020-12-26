use crate::cli_error::CliError;
use clap::{App, ArgMatches, SubCommand};
use image::ImageFormat;
use s7::algorithm::decode::decode;
use s7::serialization::read::read;

pub fn definition<'a>() -> App<'a, 'a> {
    SubCommand::with_name("decode")
        .about("Encodes input S7 to output PNG file")
        .arg(
            clap::Arg::with_name("INPUT")
                .help("Sets the path of the input file")
                .required(true)
                .index(1),
        )
        .arg(
            clap::Arg::with_name("OUTPUT")
                .help("Sets the path of the output file")
                .required(true)
                .index(2),
        )
}

pub fn exec(matches: &ArgMatches) -> Result<(), CliError> {
    let input_path = matches.value_of("INPUT").unwrap();
    let output_path = matches.value_of("OUTPUT").unwrap();

    let entity = read(input_path).unwrap();
    let image = decode(entity);

    image
        .save_with_format(output_path, ImageFormat::Png)
        .unwrap();

    Ok(())
}

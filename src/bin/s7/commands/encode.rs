use crate::cli_error::CliError;
use clap::{App, ArgMatches, SubCommand};
use s7::algorithm::encode::encode;
use s7::read_png::read_png;
use s7::serialization::write::write;

pub fn definition<'a>() -> App<'a, 'a> {
    SubCommand::with_name("encode")
        .about("Encodes input PNG to output S7 file")
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

    let input_img = read_png(input_path);
    let entity = encode(input_img);

    write(output_path, entity);

    Ok(())
}

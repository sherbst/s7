mod decode;
mod encode;

use crate::cli_error::CliError;
use clap::{App, AppSettings};

pub fn definition<'a>() -> App<'a, 'a> {
    let subcommands = vec![decode::definition(), encode::definition()];

    App::new("S7 Screenshot Serialization")
        .version("0.1.0")
        .author("Sawyer Herbst <contact@sawyerherbst.com>")
        .about("Commands for dealing with the S4 file format")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommands(subcommands)
}

pub fn exec() -> Result<(), CliError> {
    let app = definition();

    let matches = app.get_matches();

    let subcommand_name = matches.subcommand_name().unwrap();
    let subcommand_matches = matches.subcommand_matches(subcommand_name).unwrap();

    let exec_fn = match subcommand_name {
        "decode" => decode::exec,
        "encode" => encode::exec,

        _ => unreachable!(),
    };

    exec_fn(subcommand_matches)
}

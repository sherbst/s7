mod debug;
mod decode;
mod encode;

use crate::cli_error::CliError;
use clap::{App, AppSettings, Arg};
use simplelog::{ConfigBuilder, LevelFilter, TermLogger, TerminalMode};

pub fn definition<'a>() -> App<'a, 'a> {
    let subcommands = vec![
        decode::definition(),
        encode::definition(),
        debug::definition(),
    ];

    let mut subcommands_with_args: Vec<App> = Vec::new();

    for sub in subcommands {
        subcommands_with_args.push(
            sub.arg(
                Arg::with_name("verbose")
                    .multiple(true)
                    .short("v")
                    .help("verbosity level"),
            ),
        );
    }

    App::new("S7 Screenshot Serialization")
        .version("0.1.0")
        .author("Sawyer Herbst <contact@sawyerherbst.com>")
        .about("Commands for dealing with the S4 file format")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommands(subcommands_with_args)
}

pub fn exec() -> Result<(), CliError> {
    let app = definition();

    let matches = app.get_matches();

    let subcommand_name = matches.subcommand_name().unwrap();
    let subcommand_matches = matches.subcommand_matches(subcommand_name).unwrap();

    let verbosity_level = subcommand_matches.occurrences_of("verbose");
    let log_level = match verbosity_level {
        0 => LevelFilter::Info,
        1 => LevelFilter::Debug,
        2 => LevelFilter::Trace,
        _ => LevelFilter::Trace,
    };

    TermLogger::init(
        log_level,
        ConfigBuilder::new()
            .set_time_format_str("%H:%M:%S.%f")
            .build(),
        TerminalMode::Stderr,
    )
    .unwrap();

    let exec_fn = match subcommand_name {
        "decode" => decode::exec,
        "encode" => encode::exec,
        "debug" => debug::exec,

        _ => unreachable!(),
    };

    exec_fn(subcommand_matches)
}

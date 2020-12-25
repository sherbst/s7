use crate::cli_error::CliError;
use clap::{App, ArgMatches, SubCommand};

pub fn definition<'a>() -> App<'a, 'a> {
    SubCommand::with_name("decode")
}

pub fn exec(matches: &ArgMatches) -> Result<(), CliError> {
    unimplemented!()
}

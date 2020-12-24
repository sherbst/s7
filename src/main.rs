mod decode;
mod encode;
mod image;
mod io;

use encode::encode;
use io::{read_png, write_s4};

fn main() {
    dotenv::dotenv().unwrap(); // Load .env file
    pretty_env_logger::init_custom_env("DEBUG_LOG_LEVEL");

    let encode_subcommand = clap::SubCommand::with_name("encode")
        .about("Encodes input PNG to output S4 file")
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
        );

    let args = clap::App::new("S4 Screenshot Serialization")
        .version("0.1.0")
        .author("Sawyer Herbst <contact@sawyerherbst.com>")
        .about("Serializes screenshots to and from .s4 files")
        .subcommand(encode_subcommand)
        .get_matches();

    match args.subcommand_name().unwrap() {
        "encode" => {
            let encode_args = args.subcommand_matches("encode").unwrap();

            let input = read_png(encode_args.value_of("INPUT").unwrap());
            let paths = encode(input);

            write_s4(encode_args.value_of("OUTPUT").unwrap(), paths);
        }
        "decode" => unimplemented!(),
        _ => (),
    };
}

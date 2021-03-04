# s7

Lossless image compression designed to quickly compress screenshots or similar images. Currently only supports serialization to/from PNG images.

## Getting Started

### Setup

```
$ git clone https://github.com/sherbst/s7.git
$ cargo build --release
$ ./target/release/s7 help
S7 Screenshot Serialization 0.1.0
Sawyer Herbst <contact@sawyerherbst.com>
Commands for dealing with the S7 file format

USAGE:
    s7 [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    debug
    decode    Encodes input S7 to output PNG file
    encode    Encodes input PNG to output S7 file
    help      Prints this message or the help of the given subcommand(s)
```

### Encode

```
$ ./target/release/s7 encode <INPUT> <OUTPUT>
```

### Decode

```
$ ./target/release/s7 decode <INPUT> <OUTPUT>
```

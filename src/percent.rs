use clap::ArgMatches;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

use std::{
    fs,
    io::{self, Read, Write},
};

pub fn percent_encode(matches: &ArgMatches) {
    let input = match matches.value_of("input") {
        Some(path) => fs::read_to_string(path).expect("cannot read input file"),
        None => {
            let mut buffer = String::new();
            io::stdin()
                .read_to_string(&mut buffer)
                .expect("unable to read stdin");
            buffer
        }
    };
    let output = utf8_percent_encode(&input, NON_ALPHANUMERIC).to_string();
    let mut out = io::stdout();
    out.write_all(&output.as_bytes())
        .expect("unable writing into stdout");
}

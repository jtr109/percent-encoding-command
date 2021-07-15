use clap::ArgMatches;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

use std::{
    fs,
    io::{self, Read},
};

pub fn percent_encode(matches: &ArgMatches) -> String {
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
    utf8_percent_encode(&input, NON_ALPHANUMERIC).to_string()
}

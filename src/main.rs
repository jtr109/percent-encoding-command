use anyhow::Result;
use clap::{load_yaml, App};

use percent_encoding_command::{decode, encode};
use std::io::{self, Read, Write};

fn main() -> Result<()> {
    let yaml = load_yaml!("cli.yml");
    let app = App::from(yaml).version(env!("CARGO_PKG_VERSION"));
    let matches = app.clone().get_matches();

    let mut reader: Box<dyn Read> = match matches.value_of("input") {
        Some(path) => {
            let file = std::fs::OpenOptions::new().read(true).open(path)?;
            Box::new(file)
        }
        None => Box::new(io::stdin()),
    };
    let mut writer: Box<dyn Write> = match matches.value_of("output") {
        Some(path) => {
            let file = std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .open(path)?;
            Box::new(file)
        }
        None => Box::new(io::stdout()),
    };
    if matches.is_present("decode") {
        decode(&mut reader, &mut writer)?;
    } else {
        encode(&mut reader, &mut writer)?;
    }
    Ok(())
}

// TODO: https://rust-cli.github.io/book/tutorial/testing.html

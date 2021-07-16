use anyhow::Result;
use clap::{load_yaml, App};
use percent_encoding::{percent_decode_str, utf8_percent_encode, NON_ALPHANUMERIC};
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
                .create_new(true)
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

fn encode(reader: &mut dyn Read, writer: &mut dyn Write) -> Result<()> {
    let mut input = String::new();
    reader.read_to_string(&mut input)?;
    let encoded = utf8_percent_encode(&input, NON_ALPHANUMERIC).to_string();
    writer.write(encoded.as_bytes())?;
    Ok(())
}

fn decode(reader: &mut dyn Read, writer: &mut dyn Write) -> Result<()> {
    let mut input = String::new();
    reader.read_to_string(&mut input)?;
    let decoded = percent_decode_str(&input).decode_utf8()?.to_string();
    writer.write(decoded.as_bytes())?;
    Ok(())
}

// TODO: https://rust-cli.github.io/book/tutorial/testing.html

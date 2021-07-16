use anyhow::{Context, Result};
use clap::{load_yaml, App};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use std::io::{self, Read, Write};

fn main() -> Result<()> {
    let yaml = load_yaml!("cli.yml");
    let app = App::from(yaml);
    let matches = app.clone().get_matches();

    let mut reader: Box<dyn Read> = match matches.value_of("input") {
        Some(path) => {
            let file = std::fs::OpenOptions::new().read(true).open(path)?;
            Box::new(file)
        }
        None => Box::new(io::stdin()),
    };
    let mut input = String::new();
    reader.read_to_string(&mut input)?;

    let result = utf8_percent_encode(&input, NON_ALPHANUMERIC).to_string();

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
    writer.write(result.as_bytes())?;
    Ok(())
}

// fn help_info(app: &mut App) -> String {
//     let mut help = Vec::new();
//     app.write_help(&mut help).expect("unable to write help");
//     str::from_utf8(&help)
//         .expect("unable converting help into string")
//         .to_owned()
// }

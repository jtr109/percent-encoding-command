use anyhow::{Context, Result};
use clap::{load_yaml, App};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use std::{
    fs,
    io::{self, Read, Write},
};

fn main() -> Result<()> {
    let yaml = load_yaml!("cli.yml");
    let app = App::from(yaml);
    let matches = app.clone().get_matches();

    let input = match matches.value_of("input") {
        Some(path) => {
            fs::read_to_string(path).with_context(|| format!("cannot read input file {}", path))?
        }
        None => {
            let mut buffer = String::new();
            io::stdin()
                .read_to_string(&mut buffer)
                .context("unable to read stdin")?;
            buffer
        }
    };
    let output = utf8_percent_encode(&input, NON_ALPHANUMERIC).to_string();
    let mut out = io::stdout();
    out.write_all(&output.as_bytes())
        .context("unable writing into stdout")?;
    Ok(())
}

// fn help_info(app: &mut App) -> String {
//     let mut help = Vec::new();
//     app.write_help(&mut help).expect("unable to write help");
//     str::from_utf8(&help)
//         .expect("unable converting help into string")
//         .to_owned()
// }

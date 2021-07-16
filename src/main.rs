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
        Some(path) => fs::read_to_string(path)
            .with_context(|| format!("failed reading input file {}", path))?,
        None => {
            let mut buffer = String::new();
            io::stdin()
                .read_to_string(&mut buffer)
                .context("failed reading from stdin")?;
            buffer
        }
    };
    let result = utf8_percent_encode(&input, NON_ALPHANUMERIC).to_string();

    let mut output: Box<dyn Write> = match matches.value_of("output") {
        Some(path) => {
            let file = std::fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(path)?;
            Box::new(file)
        }
        None => {
            Box::new(io::stdout())
            // let mut buffer = String::new();
            // io::stdin()
            //     .read_to_string(&mut buffer)
            //     .context("failed reading from stdin")?;
            // buffer
        }
    };
    output.write(result.as_bytes())?;
    // let mut out = io::stdout();
    // out.write_all(&result.as_bytes())
    //     .context("failed writing into stdout")?;
    Ok(())
}

// fn help_info(app: &mut App) -> String {
//     let mut help = Vec::new();
//     app.write_help(&mut help).expect("unable to write help");
//     str::from_utf8(&help)
//         .expect("unable converting help into string")
//         .to_owned()
// }

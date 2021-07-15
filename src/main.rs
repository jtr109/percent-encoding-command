use buz::percent::percent_encode;
use clap::{load_yaml, App};
use std::io::{self, Write};
use std::str;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let mut app = App::from(yaml);
    let matches = app.clone().get_matches();

    let output = match matches.subcommand() {
        Some(("percent", sub_m)) => percent_encode(sub_m),
        _ => help_info(&mut app),
    };
    let mut out = io::stdout();
    out.write_all(&output.as_bytes())
        .expect("unable writing into stdout");
}

fn help_info(app: &mut App) -> String {
    let mut help = Vec::new();
    app.write_help(&mut help).expect("unable to write help");
    str::from_utf8(&help)
        .expect("unable converting help into string")
        .to_owned()
}

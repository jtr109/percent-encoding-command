use buz::percent::percent_encode;
use clap::{load_yaml, App};
use std::str;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let app = App::from(yaml);
    let matches = app.clone().get_matches();

    match matches.subcommand() {
        Some(("percent", sub_m)) => percent_encode(sub_m),
        _ => eprintln!("{}: SUBCOMMAND is required", env!("CARGO_PKG_NAME")),
    };
}

// fn help_info(app: &mut App) -> String {
//     let mut help = Vec::new();
//     app.write_help(&mut help).expect("unable to write help");
//     str::from_utf8(&help)
//         .expect("unable converting help into string")
//         .to_owned()
// }

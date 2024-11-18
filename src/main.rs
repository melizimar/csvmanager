mod commands; // Importa os subcomandos

use clap::{Arg, Command};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("CSV MANAGER")
        .version("1.0")
        .author("Matheus <melizimar@gmail.com.com>")
        .about("Ferramenta CLI para manipulação de arquivos CSV")
        .subcommand(commands::transform::command()) // Subcomando "transform"
        .subcommand(commands::split::command()) // Subcomando "splitter"
        .subcommand(commands::join::command()) // Subcomando "join"
        .get_matches();

    // Processa o subcomando invocado
    match matches.subcommand() {
        Some(("transform", sub_matches)) => commands::transform::run(sub_matches)?,
        Some(("splitter", sub_matches)) => commands::split::run(sub_matches)?,
        Some(("join", sub_matches)) => commands::join::run(sub_matches)?,
        _ => println!("Use --help para mais informações."),
    }

    Ok(())
}

use clap::{Arg, ArgMatches, Command};
use std::error::Error;

pub fn command() -> Command {
    Command::new("split")
        .about("Divide um arquivo CSV em partes menores")
        .arg(
            Arg::new("input")
                .help("Arquivo CSV de entrada")
                .short('i')
                .long("input")
                .value_name("FILE")
                .required(true),
        )
        .arg(
            Arg::new("lines")
                .help("Número de linhas por arquivo")
                .short('l')
                .long("lines")
                .value_name("LINES")
                .required(true),
        )
}

pub fn run(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let input = matches.get_one::<String>("input").unwrap();
    let lines: usize = matches
        .get_one::<String>("lines")
        .unwrap()
        .parse()
        .expect("Número inválido");

    println!("Dividindo arquivo: {} em partes de {} linhas", input, lines);
    // Implementar lógica de divisão...
    Ok(())
}

use std::error::Error;
use clap::{Arg, ArgMatches, Command};

pub fn command() -> Command {
    Command::new("split")
        .about("Divide um arquivo CSV em partes menores")
        .arg(
            Arg::new("input")
                .help("Arquivo CSV de entrada")
                .required(true),
        )
        .arg(
            Arg::new("lines")
                .help("Número de linhas por arquivo")
                .short('l')
                .long("lines")
                .required(true),
        )
}

pub fn run(matches: &ArgMatches) -> Result<(), Box<dyn Error>>{
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

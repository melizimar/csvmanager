use std::error::Error;
use std::path::{Path, PathBuf};
use std::process;

use clap::{Arg, ArgMatches, Command};

pub fn command() -> Command {
    Command::new("split")
        .about("Divide um arquivo CSV em partes menores")
        // .arg(
        //     Arg::new("split_input")
        //         .short('i')
        //         .long("input")
        //         .help("Arquivo CSV de entrada")
        //         .value_name("FILE")
        //         .required(true),
        // )
        // .arg(
        //     Arg::new("split_output")
        //         .short('o')
        //         .long("output")
        //         .help("Arquivo CSV de saída")
        //         .value_name("FILE")
        //         .required(true),
        // )
        // .arg(
        //     Arg::new("split_delimiter")
        //         .short('d')
        //         .long("delimiter")
        //         .help("Delimitador do arquivo csv")
        //         .default_value(";")
        //         .required(false),
        // )
}

pub fn run(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let input = matches.get_one::<PathBuf>("split_input").unwrap();
    let output = matches.get_one::<PathBuf>("split_output").unwrap();
    let delimiter = matches.get_one::<char>("split_delimiter").unwrap();

    if !input.exists() {
        println!("O arquivo não existe, por gentileza informe um arquivo válido.");
        process::exit(1);
    }

    Ok(())
}

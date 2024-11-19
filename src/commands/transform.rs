use std::error::Error;
use std::path::{Path, PathBuf};
use std::process;

use clap::{Arg, ArgMatches, Command};

pub fn command() -> Command {
    Command::new("transform")
        .about("Realiza transformações no arquivo CSV")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .help("Arquivo CSV de entrada")
                .required(true),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .help("Arquivo CSV de saída")
                .required(true),
        )
        .arg(
            Arg::new("delimiter")
                .short('d')
                .long("delimiter")
                .help("Delimitador do arquivo csv")
                .default_value(";")
                .required(false),
        )
        .arg(
            Arg::new("uppercase")
                .long("uppercase")
                .help("Converte todas as letras dos campos informados para maiúsculas")
                .num_args(1..)
                .required(false),
        )
        .arg(
            Arg::new("lowercase")
                .long("lowercase")
                .help("Converte todas as letras dos campos informados para minúsculas")
                .num_args(1..)
                .required(false),
        )
        .arg(
            Arg::new("normalize")
                .long("normalize")
                .help("Remove todos os acentos das letras dos campos informados")
                .num_args(1..)
                .required(false),
        )
}

pub fn run(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let input = matches.get_one::<PathBuf>("input").unwrap();
    let output = matches.get_one::<PathBuf>("output").unwrap();
    let delimiter = matches.get_one::<char>("delimiter").unwrap();

    let uppercase: Option<Vec<String>> = match matches.get_many::<String>("uppercase") {
        Some(values) => Some(values.cloned().collect()),
        None => None,
    };

    let lowercase: Option<Vec<String>> = match matches.get_many::<String>("lowercase") {
        Some(values) => Some(values.cloned().collect()),
        None => None,
    };

    let normalize: Option<Vec<String>> = match matches.get_many::<String>("normalize") {
        Some(values) => Some(values.cloned().collect()),
        None => None,
    };

    match uppercase {
        Some(fields) => println!("Campos a serem transformados em uppercase {:?}", fields),
        None => (),
    }

    match lowercase {
        Some(fields) => println!("Campos a serem transformados em lowercase {:?}", fields),
        None => (),
    }

    match normalize {
        Some(fields) => println!("Campos a serem transformados em normalize {:?}", fields),
        None => (),
    }

    if !input.exists() {
        println!("O arquivo não existe, por gentileza informe um arquivo válido.");
        process::exit(1);
    }

    Ok(())
}

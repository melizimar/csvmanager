use std::error::Error;
use std::path::{Path, PathBuf};
use std::process;
use std::thread;
use std::time::Duration;

use clap::{Arg, ArgMatches, Command};
use indicatif::{ProgressBar, ProgressStyle};

pub fn command() -> Command {
    Command::new("split").about("Divide um arquivo CSV em partes menores")
    .arg(
        Arg::new("input")
            .short('i')
            .long("input")
            .help("Arquivo CSV de entrada")
            .value_name("FILE")
            .required(true),
    )
    .arg(
        Arg::new("output")
            .short('o')
            .long("output")
            .help("Arquivo CSV de saída")
            .value_name("DIR")
            .required(true),
    )
    .arg(
        Arg::new("delimiter")
            .short('d')
            .long("delimiter")
            .help("Delimitador do arquivo csv")
            .default_value(";")
            .value_name("DELIMITER")
            .required(false),
    )
    .arg(
        Arg::new("lines")
            .short('l')
            .long("lines")
            .help("Quantidade de linhas a serem dividido os novos arquivos gerados")
            .value_name("NUM_FILES")
            .required(true),
    )
}

pub fn run(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let input = matches.get_one::<String>("input").unwrap();
    let output = matches.get_one::<String>("output").unwrap();
    let delimiter = matches.get_one::<String>("delimiter").unwrap();
    let lines = matches.get_one::<String>("lines").unwrap();

    if !PathBuf::from(input).exists() {
        println!("O arquivo não existe, por gentileza informe um arquivo válido.");
        process::exit(1);
    }

    // Processamento
    let progress_bar = ProgressBar::new(10);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("Juntando os arquivos...{pos}/{len}\n[{elapsed_precise}] [{wide_bar}] ({percent}%) ",)
            .unwrap(),
    );

    for i in 1..10 {
        progress_bar.inc(1);
        thread::sleep(Duration::from_millis(100));
    }

    progress_bar.finish_with_message("Todos os arquivos foram processados.");

    Ok(())
}

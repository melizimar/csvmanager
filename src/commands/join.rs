use std::error::Error;
use std::path::{Path, PathBuf};
use std::process;
use std::thread;
use std::time::Duration;

use clap::{Arg, ArgMatches, Command};
use indicatif::{ProgressBar, ProgressStyle};

pub fn command() -> Command {
    Command::new("join")
        .about("Une dois ou mais arquivos CSV em um único arquivo")
        .arg(
            Arg::new("inputs")
                .short('i')
                .long("inputs")
                .help("Arquivo CSV de entrada")
                .value_name("FILE, FILE, ...")
                .num_args(2..)
                .required(true),
        )
    .arg(
        Arg::new("output")
            .short('o')
            .long("output")
            .help("Arquivo CSV de saída")
            //.value_name("FILE")
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
}

pub fn run(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let inputs: Option<Vec<String>> = match matches.get_many::<String>("inputs") {
        Some(values) => Some(values.cloned().collect()),
        None => None,
    };

    let output = matches.get_one::<String>("output").unwrap();
    let delimiter = matches.get_one::<String>("delimiter").unwrap();

    // Valida se os caminhos informados no parametro inputs são caminhos válidos
    if let Some(paths) = &inputs {
        for path in paths {
            if !PathBuf::from(path).exists() {
                println!(
                    "O arquivo {:?} não existe, por gentileza informe um arquivo válido.",
                    path
                );
                process::exit(1);
            }
        }
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

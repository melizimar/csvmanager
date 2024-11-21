use std::error::Error;
use std::path::PathBuf;
use std::process;
use std::thread;
use std::time::Duration;

use clap::{Arg, ArgMatches, Command};
use indicatif::{ProgressBar, ProgressStyle};

pub fn command() -> Command {
    Command::new("transform")
        .about("Realiza transformações no arquivo CSV")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .help("Arquivo CSV de entrada")
                //.value_name("FILE")
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
    let input = matches.get_one::<String>("input").unwrap();
    let _output = matches.get_one::<String>("output").unwrap();
    let _delimiter = matches.get_one::<String>("delimiter").unwrap();

    let uppercase: Option<Vec<String>> = matches
        .get_many::<String>("uppercase")
        .map(|values| values.cloned().collect());
    let lowercase: Option<Vec<String>> = matches
        .get_many::<String>("lowercase")
        .map(|values| values.cloned().collect());
    let normalize: Option<Vec<String>> = matches
        .get_many::<String>("normalize")
        .map(|values| values.cloned().collect());

    // match uppercase {
    //     Some(fields) => println!("Campos a serem transformados em uppercase {:?}", fields),
    //     None => (),
    // }

    if let Some(fields) = uppercase {
        println!("Campos a serem transformados em uppercase {:?}", fields)
    }

    // match lowercase {
    //     Some(fields) => println!("Campos a serem transformados em lowercase {:?}", fields),
    //     None => (),
    // }

    if let Some(fields) = lowercase {
        println!("Campos a serem transformados em lowercase {:?}", fields)
    }

    // match normalize {
    //     Some(fields) => println!("Campos a serem transformados em normalize {:?}", fields),
    //     None => (),
    // }

    if let Some(fields) = normalize {
        println!("Campos a serem transformados em normalize {:?}", fields)
    }

    if !PathBuf::from(input).exists() {
        println!("O arquivo não existe, por gentileza informe um arquivo válido.");
        process::exit(1);
    }

    // Processamento
    let progress_bar = ProgressBar::new(10);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template(
                "Transformando arquivo...{pos}/{len}\n[{elapsed_precise}] [{wide_bar}] ({percent}%) ",
            )
            .unwrap(),
    );

    for _i in 1..10 {
        progress_bar.inc(1);
        thread::sleep(Duration::from_millis(100));
    }

    progress_bar.finish_with_message("Todo o arquivo foi processado.");

    Ok(())
}

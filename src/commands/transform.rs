use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;
use std::process;
use std::thread;
use std::time::Duration;

use clap::{Arg, ArgMatches, Command};
use indicatif::{ProgressBar, ProgressStyle};
use polars::prelude::*;

#[warn(unused_imports)]
use deunicode::deunicode;

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
    let delimiter = matches.get_one::<String>("delimiter").unwrap();
    let delimiter_u8: u8 = delimiter.as_bytes()[0];

    if !PathBuf::from(input).exists() {
        println!("O arquivo não existe, por gentileza informe um arquivo válido.");
        process::exit(1);
    }

    // Configurando o spinner
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg} {elapsed_precise}")
            .unwrap()
            .tick_strings(&["-", "\\", "|", "/"]),
    );
    spinner.enable_steady_tick(Duration::from_millis(100));

    spinner.set_message("Lendo o arquivo...");
    let mut lf = LazyCsvReader::new(input)
        .with_has_header(true)
        .with_separator(delimiter_u8)
        .with_ignore_errors(true)
        .finish()?;

    spinner.set_message("Processando transformações...");
    // Capturando a sequência dos argumentos inseridos
    let transform_order: Vec<_> = matches.ids().map(|id| id.as_str()).collect();
    for arg in transform_order {
        match arg {
            "uppercase" => {
                let uppercase: Option<Vec<String>> = matches
                    .get_many::<String>("uppercase")
                    .map(|values| values.cloned().collect());

                if let Some(fields) = uppercase {
                    let uppercase_transformations: Vec<Expr> = fields
                        .iter()
                        .map(|col_name| col(col_name).str().to_uppercase().alias(col_name))
                        .collect();
                    lf = lf.with_columns(uppercase_transformations);
                }
            }
            "lowercase" => {
                let lowercase: Option<Vec<String>> = matches
                    .get_many::<String>("lowercase")
                    .map(|values| values.cloned().collect());

                if let Some(fields) = lowercase {
                    let lowercase_transformations: Vec<Expr> = fields
                        .iter()
                        .map(|col_name| col(col_name).str().to_lowercase().alias(col_name))
                        .collect();
                    lf = lf.with_columns(lowercase_transformations);
                }
            }
            "normalize" => {
                let normalize: Option<Vec<String>> = matches
                    .get_many::<String>("normalize")
                    .map(|values| values.cloned().collect());

                if let Some(fields) = normalize {
                    println!("Implementar normalize, {:?}", fields);
                }
            }
            _ => (),
        }
    }

    spinner.set_message("Coletando o DataFrame...");
    let mut df = lf.collect()?;
    let csv_length = u64::try_from(df.height())?;

    let mut file = File::create(_output).unwrap();

    spinner.set_message("Gravando o arquivo de saída...");
    CsvWriter::new(&mut file)
        .include_header(true)
        .with_separator(delimiter_u8)
        .finish(&mut df)
        .unwrap();

    // Finaliza o spinner com uma mensagem
    spinner.finish_with_message("Processamento concluído!");

    print!("\x1B[2J\x1B[1;1H"); // Limpa o terminal

    Ok(())
}

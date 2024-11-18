use std::error::Error;

use clap::{Arg, ArgMatches, Command};

pub fn command() -> Command {
    Command::new("join")
        .about("Une dois ou mais arquivos CSV em um único arquivo")
        .arg(
            Arg::new("inputs")
                .help("Arquivos CSV de entrada")
                .required(true)
                .num_args(1..)  // Permite múltiplos arquivos de entrada
        )
        .arg(
            Arg::new("output")
                .help("Arquivo CSV de saída")
                .required(true),
        )
}

pub fn run(matches: &ArgMatches) -> Result<(), Box<dyn Error>>{
    // Usando `get_many` para capturar múltiplos valores para o argumento `inputs`
    let inputs: Vec<String> = matches.get_many::<String>("inputs")
        .unwrap()
        .cloned()
        .collect();

    let output = matches.get_one::<String>("output").unwrap();

    println!("Unindo arquivos: {:?} em {}", inputs, output);
    // Implementar lógica de união...

    Ok(())
}

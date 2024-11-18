use clap::{Arg, ArgMatches, Command};
use std::error::Error;

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
            Arg::new("uppercase")
                .short('u')
                .long("uppercase")
                .help("Converte todas as letras para maiúsculas")
                .num_args(1..)
                .required(false),
        )
}

pub fn run(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let input = matches.get_one::<String>("input").unwrap();
    let output = matches.get_one::<String>("output").unwrap();
    //let uppercase = matches.get_flag("uppercase");
    let uppercase_fields: Vec<String> = matches.get_many("uppercase")
        .unwrap()
        .cloned()
        .collect();

    println!("Transformando arquivo: {}", input);
    println!("Arquivo de saída: {}", output);

    match uppercase_fields {
        Ok(vec) => println!("Transformação: Maiúsculas ativadas! {:?}", vec),
        _ => println!("Sem valor"),
        // Chamar função para processar...
    }

    Ok(())
}

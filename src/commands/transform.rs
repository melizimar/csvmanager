use clap::{Arg, ArgMatches, Command};

pub fn command() -> Command {
    Command::new("transform")
        .about("Realiza transformações no arquivo CSV")
        .arg(
            Arg::new("input")
                .about("Arquivo CSV de entrada")
                .required(true),
        )
        .arg(
            Arg::new("output")
                .about("Arquivo CSV de saída")
                .required(true),
        )
        .arg(
            Arg::new("uppercase")
                .about("Converte todas as letras para maiúsculas")
                .short('u')
                .long("uppercase")
                .takes_value(false),
        )
}

pub fn run(matches: &ArgMatches) {
    let input = matches.value_of("input").unwrap();
    let output = matches.value_of("output").unwrap();
    let uppercase = matches.is_present("uppercase");

    println!("Transformando arquivo: {}", input);
    println!("Arquivo de saída: {}", output);

    if uppercase {
        println!("Transformação: Maiúsculas ativadas!");
        // Chamar função para processar...
    }
}
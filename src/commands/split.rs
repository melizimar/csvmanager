use clap::{Arg, ArgMatches, Command};

pub fn command() -> Command {
    Command::new("split")
        .about("Divide um arquivo CSV em partes menores")
        .arg(
            Arg::new("input")
                .about("Arquivo CSV de entrada")
                .required(true),
        )
        .arg(
            Arg::new("lines")
                .about("Número de linhas por arquivo")
                .short('l')
                .long("lines")
                .takes_value(true)
                .required(true),
        )
}

pub fn run(matches: &ArgMatches) {
    let input = matches.value_of("input").unwrap();
    let lines: usize = matches
        .value_of("lines")
        .unwrap()
        .parse()
        .expect("Número inválido");

    println!("Dividindo arquivo: {} em partes de {} linhas", input, lines);
    // Implementar lógica de divisão...
}

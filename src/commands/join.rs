use clap::{Arg, ArgMatches, Command};

pub fn command() -> Command {
    Command::new("join")
        .about("Une dois ou mais arquivos CSV em um único arquivo")
        .arg(
            Arg::new("inputs")
                .about("Arquivos CSV de entrada")
                .required(true)
                .multiple_values(true),
        )
        .arg(
            Arg::new("output")
                .about("Arquivo CSV de saída")
                .required(true),
        )
}

pub fn run(matches: &ArgMatches) {
    let inputs: Vec<_> = matches.values_of("inputs").unwrap().collect();
    let output = matches.value_of("output").unwrap();

    println!("Unindo arquivos: {:?} em {}", inputs, output);
    // Implementar lógica de união...
}

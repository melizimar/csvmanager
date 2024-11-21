use std::error::Error;
use std::path::{Path, PathBuf};
use std::process;

use clap::{Arg, ArgMatches, Command};

pub fn command() -> Command {
    Command::new("join")
        .about("Une dois ou mais arquivos CSV em um único arquivo")
        // .arg(
        //     Arg::new("join_inputs")
        //         .short('i')
        //         .long("inputs")
        //         .help("Arquivo CSV de entrada")
        //         .value_name("FILE, FILE, ...")
        //         .num_args(1..)
        //         .required(true),
        // )
        // .arg(
        //     Arg::new("join_output")
        //         .short('o')
        //         .long("output")
        //         .help("Arquivo CSV de saída")
        //         //.value_name("FILE")
        //         .required(true),
        // )
        // .arg(
        //     Arg::new("join_delimiter")
        //         .short('d')
        //         .long("delimiter")
        //         .help("Delimitador do arquivo csv")
        //         .default_value(";")
        //         .required(false),
        // )

}

pub fn run(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    // let inputs: Option<Vec<PathBuf>> = match matches.get_many::<PathBuf>("inputs") {
    //     Some(values) => Some(values.cloned().collect()),
    //     None => None,
    // };

    //let output = matches.get_one::<PathBuf>("output").unwrap();
    //let delimiter = matches.get_one::<char>("delimiter").unwrap();

    // if let Some(paths) = inputs {
    //     for path in paths {
    //         if !path.exists() {
    //             println!("O arquivo {:?} não existe, por gentileza informe um arquivo válido.", path);
    //             process::exit(1);
    //         }
    //     }
    // }

    println!("Join");

    Ok(())
}

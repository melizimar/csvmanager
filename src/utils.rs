use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn count_lines_in_file(file_path: &str) -> io::Result<usize> {
    // Abre o arquivo para leitura
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Conta as linhas iterando sobre elas
    let line_count = reader.lines().count();

    Ok(line_count)
}


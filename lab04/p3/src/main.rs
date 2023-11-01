use std::io::Write;
use std::{fs, io};
fn replace(input: &str) -> String {
    let mut result = String::new();

    for word in input.split_whitespace() {
        let replacement = match word {
            "pt" => "pentru",
            "ptr" => "pentru",
            "dl" => "domnul",
            "dna" => "doamna",
            _ => word,
        };
        result.push_str(replacement);
        result.push(' ');
    }
    result
}
fn verify() -> Result<(), io::Error> {
    let s = fs::read_to_string("input.txt")?;

    let mut output_file = fs::File::create("output.txt")?;

    for line in s.lines() {
        let replaced_line = replace(&line);
        writeln!(output_file, "{}", replaced_line)?;
    }

    Ok(())
}
fn main() {
    match verify() {
        Ok(()) => {
            println!("Am modificat!");
        }

        Err(err) => {
            eprintln!("Error reading the file: {}", err);
        }
    }
}

use std::{fs, io};

fn find_longest_lines() -> Result<(String, String), io::Error> {
    let s = fs::read_to_string("example.txt")?;
    let mut longest_line_bytes = String::new();
    let mut longest_line_characters = String::new();
    let mut max_bytes = 0;
    let mut max_characters = 0;

    for i in s.lines() {
        if i.len() > max_bytes {
            max_bytes = i.len();
            longest_line_bytes = i.clone().to_string();
        }

        if i.chars().count() > max_characters {
            max_characters = i.chars().count();
            longest_line_characters = i.to_string();
        }
    }

    Ok((longest_line_bytes, longest_line_characters))
}

fn main() {
    match find_longest_lines() {
        Ok((longest_bytes, longest_characters)) => {
            println!("Longest line in bytes:");
            println!("{}", longest_bytes);
            println!("Longest line in characters:");
            println!("{}", longest_characters);
        }
        Err(err) => {
            eprintln!("Error reading the file: {}", err);
        }
    }
}

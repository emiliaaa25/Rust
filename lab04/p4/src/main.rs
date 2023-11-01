use std::{fs, io};

fn print() -> Result<(), io::Error> {
    let s = fs::read_to_string("hosts.txt")?;

    for line in s.lines() {
        let line = line;
        if line.starts_with('#') {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            let hostname = parts[0];
            let ip_address = parts[1];
            println!("{} => {}", hostname, ip_address);
        }
    }

    Ok(())
}

fn main() {
    match print() {
        Ok(()) => {}

        Err(err) => {
            eprintln!("Error reading the file: {}", err);
        }
    }
}

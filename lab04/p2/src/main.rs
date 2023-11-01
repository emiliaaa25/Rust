use std::io;

fn rotated(c: char) -> char {
    match c {
        'a'..='z' => {
            let rotated = c as u8 + 13;
            if rotated > b'z' {
                (rotated - 26) as char
            } else {
                rotated as char
            }
        }
        'A'..='Z' => {
            let rotated = c as u8 + 13;
            if rotated > b'Z' {
                (rotated - 26) as char
            } else {
                rotated as char
            }
        }
        _ => c,
    }
}

fn rot13(input: &str) -> String {
    let mut result = String::new();
    for c in input.chars() {
        result.push(rotated(c));
    }
    result
}

fn main() {
    let mut input_text = String::new();

    io::stdin()
        .read_line(&mut input_text)
        .expect("Eroare la citirea de la intrare");

    let encrypted = rot13(&input_text);
    print!("{}\n", encrypted);
}

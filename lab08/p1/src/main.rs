use std::collections::HashMap;
use std::fs;
use std::io::{self};

fn main() -> io::Result<()> {
    let s = fs::read_to_string("file.txt.txt")?;
    let mut map = HashMap::new();

    for line in s.lines() {
        for word in line.split_whitespace() {
            let mut new_word = String::new();

            for c in word.chars() {
                if c.is_alphanumeric() {
                    new_word.push(c.to_lowercase().next().unwrap());
                }
            }

            if !new_word.is_empty() {
                let count = map.entry(new_word).or_insert(0);
                *count += 1;
            }
        }
    }

    let mut v: Vec<_> = map.into_iter().collect();
    v.sort_by(|a, b| b.1.cmp(&a.1));

    for (word, count) in v {
        println!("{:<8} => {}", word, count);
    }

    Ok(())
}

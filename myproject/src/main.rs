use std::collections::VecDeque;
use std::io;

fn lexing(fun: &str) -> VecDeque<char> {
    let mut vect = VecDeque::new();
    let mut chars = fun.chars().peekable();

    while let Some(&c) = chars.peek() {
        if c.is_ascii_digit() || c == '.' {
            vect.push_back(c);
            chars.next();
            while let Some(&next_char) = chars.peek() {
                if next_char.is_ascii_digit() || next_char == '.' {
                    vect.push_back(next_char);
                    chars.next();
                } 
                else {
                    break;
                }
            }
        } 
        else if c == '+' || c == '-' || c == '*' || c == '/' || c == '^' || c == '(' || c == ')' {
            vect.push_back(c);
            chars.next();
        }   
            else {
            match c {
                's' => {
                    if chars.peek() == Some(&'q') {
                        vect.push_back('r');
                        chars.nth(3);
                    } else {
                        vect.push_back('s');
                        chars.nth(2);
                    }
                }
                'c' => {
                    vect.push_back('c');
                    chars.nth(2);
                }
                'l' => {
                    vect.push_back('l');
                    chars.nth(1);
                }
                't' => {
                    vect.push_back('t');
                    chars.nth(2);
                }
                _ => {
                    chars.next();
                }
            }
        }
    }
    vect
}

fn main() {
    println!("Enter a mathematical expression:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading the input");

    let infixata = lexing(&input);
    println!("{:?}", infixata);
}

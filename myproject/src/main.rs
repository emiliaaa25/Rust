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

struct Stack {
    stack: Vec<char>,
}

impl Stack {
    fn new() -> Stack {
        Stack { stack: Vec::new() }
    }

    fn push(&mut self, item: char) {
        self.stack.push(item);
    }

    fn pop(&mut self) -> Option<char> {
        self.stack.pop()
    }

    fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    fn top(&self) -> Option<&char> {
        self.stack.last()
    }
}

fn parsing(infixata: &mut VecDeque<char>, postfixata: &mut VecDeque<char>, stack: &mut Stack) {
    while !infixata.is_empty() {
        if infixata[0].is_ascii_digit() || infixata[0] == '.' {
            postfixata.push_back(infixata.pop_front().unwrap());
            while let Some(&next_char) = infixata.front() {
                if next_char.is_ascii_digit() || next_char == '.' {
                    postfixata.push_back(infixata.pop_front().unwrap());
                } else {
                    break;
                }
            }
        } else if infixata[0] == ')' {
            while let Some(ch) = stack.pop() {
                if ch == '(' {
                    break;
                }
                postfixata.push_back(ch);
            }
            infixata.pop_front();
        } else {
            while !stack.is_empty() {
                postfixata.push_back(stack.pop().unwrap());
            }
            stack.push(infixata.pop_front().unwrap());
        }
    }

}

fn main() {
    println!("Enter a mathematical expression:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading the input");

    let mut infixata = lexing(&input);
    println!("{:?}", infixata);

    let mut postfixata: VecDeque<char> = VecDeque::new();
    let mut stack = Stack::new();

    parsing(&mut infixata, &mut postfixata, &mut stack);

    println!("{:?}",postfixata);
}

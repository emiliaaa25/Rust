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
struct Stack1 {
    stack1: Vec<f64>,
}

impl Stack1 {
    fn new() -> Stack1 {
        Stack1 { stack1: Vec::new() }
    }

    fn push(&mut self, item: f64) {
        self.stack1.push(item);
    }

    fn pop(&mut self) -> Option<f64> {
        self.stack1.pop()
    }
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
    while !stack.is_empty() {
        postfixata.push_back(stack.pop().unwrap());
    }

}
fn operatie(r: char, x: f64, y: f64) -> f64 {
    match r {
        '+' => x + y,
        '-' => x - y,
        '*' => x * y,
        '/' => x / y,
        '^' => x.powf(y),
        _ => panic!("Invalid operator"),
    }
}
fn operatie_speciala(r: char, x: f64) -> f64 {
    match r {
        's' => x.sin(),
        'c' => x.cos(),
        'l' => x.ln(),
        't' => x.tan(),
        'r' => x.sqrt(),
        _ => panic!("Invalid function"),
    }
}
fn resolving(postfixata: &mut VecDeque<char>, stack1: &mut Stack1) -> f64 {
    while !postfixata.is_empty() {
        if postfixata[0].is_ascii_digit() || postfixata[0] == '.' {
            let mut val_string = String::new();
            while let Some(&ch) = postfixata.front() {
                if ch.is_ascii_digit() || ch == '.' {
                    val_string.push(postfixata.pop_front().unwrap());
                }else {
                    break;
                }
            }
            let val = val_string.parse::<f64>().unwrap();
            stack1.push(val);
        } else if postfixata[0]=='+'||postfixata[0]=='-'||postfixata[0]=='*'||postfixata[0]=='/'||postfixata[0]=='^' {
            let val2 = stack1.pop().unwrap();
            let val1 = stack1.pop().unwrap();
            let operator1 = postfixata.pop_front().unwrap();
            let answ = operatie(operator1, val1, val2);
            stack1.push(answ);
        } else if postfixata[0]=='s'||postfixata[0]=='c'||postfixata[0]=='l'||postfixata[0]=='t'||postfixata[0]=='r' {
            let val1 = stack1.pop().unwrap();
            let operator1 = postfixata.pop_front().unwrap();
            let answ = operatie_speciala(operator1, val1);
            stack1.push(answ);
        }
    }

    stack1.pop().unwrap()
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
    let mut stack1 = Stack1::new();
    let result = resolving(&mut postfixata, &mut stack1);
    println!("Result: {}", result);
}
use std::collections::VecDeque;
use std::io;
use std::env;


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
                } else {
                    break;
                }
            }
        } else if c == '+' || c == '-' || c == '*' || c == '/' || c == '^' || c == '(' || c == ')' {
            vect.push_back(c);
            chars.next();
            let next_char = chars.peek();
            if next_char == Some(&c) {
                chars.next();
            }
        } else {
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
fn prioritate(c: char) -> u8 {
    if c == '+' || c == '-' {
        1
    } else if c == '*' || c == '/' {
        2
    } else if c == '^' {
        3
    } else if c == 's' || c == 'c' || c == 'l' || c == 't' {
        4
    } else if c == '(' || c == ')' {
        5
    } else {
        0
    }
}
fn parsing(infixata: &mut VecDeque<char>, postfixata: &mut VecDeque<char>, stack: &mut Stack) {
    while let Some(&c) = infixata.front() {
        if c.is_ascii_digit() || c == '.' {
            while let Some(&digit) = infixata.front() {
                if digit.is_ascii_digit() || digit == '.' {
                    postfixata.push_back(infixata.pop_front().unwrap());
                } else {
                    break;
                }
            }
            postfixata.push_back(' ');
        } else if c == '(' {
            stack.push(infixata.pop_front().unwrap());
        } else if c == ')' && stack.top() != Some(&'(') {
            while let Some(&top_char) = stack.top() {
                if top_char == '(' {
                    stack.pop();
                    break;
                }
                postfixata.push_back(stack.pop().unwrap());
            }
            infixata.pop_front();
        } else if c == ')' && stack.top() == Some(&'(') {
            stack.pop();
            postfixata.push_back(stack.pop().unwrap());
        } else if c == '+'
            || c == '-'
            || c == '*'
            || c == '/'
            || c == '^'
            || c == 's'
            || c == 'c'
            || c == 'l'
            || c == 't'
        {
            while !stack.is_empty()
                && prioritate(*stack.top().unwrap()) >= prioritate(c)
                && *stack.top().unwrap() != '('
            {
                postfixata.push_back(stack.pop().unwrap());
            }
            stack.push(infixata.pop_front().unwrap());
        } else {
            infixata.pop_front();
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
        'c' => x.to_radians().cos(),
        'l' => x.ln(),
        't' => x.to_radians().tan(),
        'r' => x.sqrt(),
        _ => panic!("Invalid function"),
    }
}
fn update_and_print_infixata1(
    infixata1: &mut VecDeque<char>,
    val1: f64,
    val2: f64,
    operator: char,
    result: f64,
) {
    let val1_str = val1.to_string();
    let val2_str = val2.to_string();
    let result_str = result.to_string();

    let replace_from = format!("{}{}{}", val1_str, operator, val2_str);
    let replace_with = result_str;

    let replace_from_chars: Vec<char> = replace_from.chars().collect();
    let replace_with_chars: Vec<char> = replace_with.chars().collect();

    for i in 0..infixata1.len() {
        if i + replace_from_chars.len() > infixata1.len() {
            break;
        }
        if infixata1
            .range(i..i + replace_from_chars.len())
            .eq(replace_from_chars.iter())
        {
            infixata1.drain(i..i + replace_from_chars.len());
            for ch in replace_with_chars.iter().rev() {
                infixata1.insert(i, *ch);
            }
            if i > 0
                && infixata1.get(i - 1) == Some(&'(')
                && infixata1.get(i + replace_with_chars.len()) == Some(&')') 
            {
                infixata1.remove(i + replace_with_chars.len());
                infixata1.remove(i - 1);
            }
            break;
        }
    }

    println!("= {}", infixata1.iter().collect::<String>());
}

fn update_and_print_infixata1_special(
    infixata1: &mut VecDeque<char>,
    operator: char,
    val1: f64,
    result: f64,
) {
    
    let val1_str = val1.to_string();
    let result_str = result.to_string();

    let replace_from = format!("{}({})", operator, val1_str);
    let replace_with = result_str;

    let replace_from_chars: Vec<char> = replace_from.chars().collect();
    let replace_with_chars: Vec<char> = replace_with.chars().collect();

    for i in 0..infixata1.len() {
        if i + replace_from_chars.len() > infixata1.len() {
            break;
        }
        if infixata1
            .range(i..i + replace_from_chars.len())
            .eq(replace_from_chars.iter())


        {
            infixata1.drain(i..i + replace_from_chars.len()-1);
            for ch in replace_with_chars.iter().rev() {
                infixata1.insert(i, *ch);
            }
        
            break;
        }   
    }

    println!("= {}", infixata1.iter().collect::<String>());
}


fn resolving(
    postfixata: &mut VecDeque<char>,
    stack1: &mut Stack1,
    infixata1: &mut VecDeque<char>,
) -> f64 {
    while !postfixata.is_empty() {
        if postfixata[0].is_ascii_digit() || postfixata[0] == '.' {
            let mut val_string = String::new();
            while let Some(&ch) = postfixata.front() {
                if ch.is_ascii_digit() || ch == '.' {
                    val_string.push(postfixata.pop_front().unwrap());
                } else if ch == ' ' {
                    postfixata.pop_front();
                    break;
                } else {
                    break;
                }
            }
            let val = val_string.parse::<f64>().unwrap();
            stack1.push(val);
        } else if postfixata[0] == '+'
            || postfixata[0] == '-'
            || postfixata[0] == '*'
            || postfixata[0] == '/'
            || postfixata[0] == '^'
        {
            let val2 = stack1.pop().unwrap_or_default();
            let val1 = stack1.pop().unwrap_or_default();
            let operator1 = postfixata.pop_front().unwrap_or_default();
            let answ = operatie(operator1, val1, val2);
            stack1.push(answ);
            update_and_print_infixata1(infixata1, val1, val2, operator1, answ);
        } else if postfixata[0] == 's'
            || postfixata[0] == 'c'
            || postfixata[0] == 'l'
            || postfixata[0] == 't'
            || postfixata[0] == 'r'
        {
            let val1 = stack1.pop().unwrap_or_default();
            let operator1 = postfixata.pop_front().unwrap();
            let answ = operatie_speciala(operator1, val1);
            stack1.push(answ);
            update_and_print_infixata1_special(infixata1, operator1, val1, answ);
        }
    }

    stack1.pop().unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut input = String::new();

    if args.len() < 2 {
        println!("Enter a mathematical expression:");
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading the input");
    
    }
    else {
        input=args[1].clone();
    }
   

    let mut infixata = lexing(&input);
    let mut infixata1 = lexing(&input);
    let mut postfixata: VecDeque<char> = VecDeque::new();
    let mut stack = Stack::new();
   
    parsing(&mut infixata, &mut postfixata, &mut stack);
    let mut stack1 = Stack1::new();
    let result = resolving(&mut postfixata, &mut stack1, &mut infixata1);
    println!("Result: {}", result);
}

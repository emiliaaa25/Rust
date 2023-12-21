use std::cell::RefCell;
use std::collections::HashMap;

struct Cache {
    values: RefCell<HashMap<u64, bool>>,
}

impl Cache {
    fn new() -> Self {
        Cache {
            values: RefCell::new(HashMap::new()),
        }
    }

    fn is_prime(&self, n: u64) -> bool {
        if n <= 1 {
            return false;
        }
        for i in 2..=(n as f64).sqrt() as u64 {
            if n % i == 0 {
                return false;
            }
        }
        true
    }

    fn get_or_insert(&self, n: u64) -> bool {
        let mut cache = self.values.borrow_mut();
        if let Some(&result) = cache.get(&n) {
            result
        } else {
            let result = self.is_prime(n);
            cache.insert(n, result);
            result
        }
    }
}

fn main() {
    let cache = Cache::new();

    loop {
        let mut input = String::new();
        println!("Enter a number or exit ");
        std::io::stdin()
            .read_line(&mut input)
            .expect("Error reading line");

        if input.trim() == "exit" {
            break;
        }

        match input.trim().parse::<u64>() {
            Ok(n) => {
                let is_prime = cache.get_or_insert(n);
                if is_prime {
                    println!("{} is prime.", n);
                } else {
                    println!("{} isn't prime.", n);
                }
            }
            Err(_) => println!("Invalid input."),
        }
    }
}

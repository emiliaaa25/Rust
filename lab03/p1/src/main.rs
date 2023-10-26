fn prime(n: u16) -> bool {
    if n < 2 {
        return false;
    }
    let mut i: u16 = 2;
    while i <= n / 2 {
        if n % i == 0 {
            return false;
        }
        i = i + 1;
    }
    return true;
}
fn next_prime(mut x: u16) -> Option<u16> {
    while x < std::u16::MAX {
        x = x + 1;
        if prime(x) == true {
            return Some(x);
        }
    }
    return None;
}
fn main() {
    let x = next_prime(5);
    match x {
        Some(value) => println!("next number is prime {value}"),
        None => println!("overflow"),
    }
    let x = next_prime(80);
    match x {
        Some(value) => println!("next number is prime {value}"),
        None => println!("overflow"),
    }

    let x = next_prime(600);
    match x {
        Some(value) => println!("next number is prime {value}"),
        None => println!("overflow"),
    }
    let x = next_prime(60000);
    match x {
        Some(value) => println!("next number is prime {value}"),
        None => println!("overflow"),
    }

    let x = next_prime(65534);
    match x {
        Some(value) => println!("next number is prime {value}"),
        None => println!("overflow"),
    }
}

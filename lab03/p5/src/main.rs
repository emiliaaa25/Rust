//cel mai mare divizor comun al unor numere mai mari decat 100 care sunt si palindroame
use thiserror::Error;

#[derive(Error, Debug)]
enum MyError {
    #[error("Niciun numar nu trebuie sa fie 0")]
    NrIs0,
    #[error("Numerele nu trebuie sa fie mai mici de 100")]
    NrSmallerThan100,
}
fn cmmdc(mut a: i32, mut b: i32) -> Result<i32, MyError> {
    if a * b == 0 {
        return Err(MyError::NrIs0);
    }
    if a < 100 || b < 100 {
        return Err(MyError::NrSmallerThan100);
    }

    while a != b {
        if a > b {
            a = a - b;
        } else {
            b = b - a;
        }
    }
    Ok(a)
}
fn oglindit(mut n: i32) -> i32 {
    let mut og: i32 = 0;
    while n > 0 {
        og = og * 10 + n % 10;
        n = n / 10;
    }
    return og;
}
fn palindrom(a: i32) -> Option<i32> {
    if oglindit(a) == a {
        return Some(a);
    }

    return None;
}
fn print_error(err: MyError) {
    match err {
        MyError::NrIs0 => eprintln!("Error: Niciun nr nu trb sa fie 0."),
        MyError::NrSmallerThan100 => {
            eprintln!("Error: Numerele  u trebuie sa fie mai mici de 100.")
        }
    }
}

fn main() {
    let mut k = 0;
    let a = palindrom(131);
    if a.is_some() {
        println!("primul numar este palindrom");
    } else {
        k = 1;
        println!("primul numar nu este palindrom");
    }
    let b = palindrom(514);
    if b.is_some() {
        println!("al doilea numar este palindrom");
    } else {
        k = 1;
        println!("al doilea numar nu este palindrom");
    }
    if k == 0 {
        let m = a.unwrap();
        let n = b.unwrap();

        match cmmdc(m, n) {
            Ok(m) => println!("Cmmdc este: {}", m),
            Err(err) => print_error(err),
        }
    }
}

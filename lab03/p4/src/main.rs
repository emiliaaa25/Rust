use thiserror::Error;

#[derive(Error, Debug)]
enum MyError 
{
    #[error("Caracterul nu e ascii")]
    NotAscii,
    #[error("Caracterul nu e cifra")]
    NotDigit,
    #[error("Caracterul nu e in baza 16")]
    NotBase16,
    #[error("Caracterul nu e litera")]
    NotLetter,
    #[error("Caracterul nu e printabil")]
    NotPrintable,
}
fn to_uppercase(c: char) -> Result<char, MyError> {
    if !c.is_alphabetic() {
        return Err(MyError::NotLetter);
    }
    let m = c.to_ascii_uppercase();
    Ok(m)
}

fn to_lowercase(c: char) -> Result<char, MyError> {
    if !c.is_alphabetic() {
        return Err(MyError::NotLetter);
    }
    let m = c.to_ascii_lowercase();
    Ok(m)
}
fn print_char(c: char) -> Result<(), MyError> {
    if !c.is_ascii_graphic() {
        return Err(MyError::NotPrintable);
    }
    println!("{}", c);
    Ok(())
}
fn char_to_number(c: char) -> Result<u32, MyError> {
    if !c.is_ascii() {
        return Err(MyError::NotAscii);
    }
    if !c.is_ascii_digit() {
        return Err(MyError::NotDigit);
    }

    let digit = c.to_digit(10).unwrap();
    Ok(digit)
}

fn char_to_number_hex(c: char) -> Result<u32, MyError> {
    if !c.is_ascii() {
        return Err(MyError::NotAscii);
    }
    if !c.is_ascii_hexdigit() {
        return Err(MyError::NotBase16);
    }

    let digit = c.to_digit(16).unwrap();
    Ok(digit)
}

fn print_error(err: MyError) {
    match err {
        MyError::NotAscii => eprintln!("Error: Caracterul nu e ascii."),
        MyError::NotDigit => eprintln!("Error: Caracterul nu e cifra."),
        MyError::NotBase16 => eprintln!("Error: Caracterul nu e in baza 16."),
        MyError::NotLetter => eprintln!("Error: Caracterul nu e litera."),
        MyError::NotPrintable => eprintln!("Error: Caracterul nu e printabil."),
    }
}

fn main() {
    match to_lowercase('X') {
        Ok(m) => println!("Lowercase: {}", m),
        Err(err) => print_error(err),
    }

    match to_uppercase('.') {
        Ok(m) => println!("Uppercase: {}", m),
        Err(err) => print_error(err),
    }

    match print_char('\n') {
        Ok(()) => println!("Printed successfully."),
        Err(err) => print_error(err),
    }

    match char_to_number('9') {
        Ok(digit) => println!("Number: {}", digit),
        Err(err) => print_error(err),
    }

    match char_to_number_hex('M') {
        Ok(digit) => println!("Hexadecimal Number: {}", digit),
        Err(err) => print_error(err),
    }

}

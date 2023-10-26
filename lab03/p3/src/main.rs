use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error("Numarul e de tip diferit")]
    Overflow,
}

fn add(x: u32, y: u32) -> Result<u32, Error> {
    let sum = x as u64 + y as u64;
    if sum > std::u32::MAX as u64 {
        return Err(Error::Overflow);
    }

    Ok(sum as u32)
}

fn multiplication(x: u32, y: u32) -> Result<u32, Error> {
    let mult = x as u64 * y as u64;
    if mult > std::u32::MAX as u64 {
        return Err(Error::Overflow);
    }
    Ok(mult as u32)
}

fn f() -> Result<u32, Error> {
    let s1 = multiplication(1500000, 10000)?;
    let s1 = add(s1, 10)?;
    Ok(s1)
}

fn main() {
    match f() {
        Ok(x) => println!("value:  {}", x),
        Err(e) => eprintln!("{:?}", e),
    }
}

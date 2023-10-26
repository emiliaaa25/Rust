fn add(x: u32, y: u32) -> u32 {
    let sum = x as u64 + y as u64;
    if sum > std::u32::MAX as u64 {
        panic!("sum isn't u32 type");
    }

    return sum as u32;
}

fn multiplication(x: u32, y: u32) -> u32 {
    let mult = x as u64 * y as u64;
    if mult > std::u32::MAX as u64 {
        panic!("multiplication isn't u32 type");
    }

    return mult as u32;
}
fn main() {
    let x = add(5, 9);
    println!("{}", x);
    let y = multiplication(300000, 30000000);
    println!("{}", y);
}

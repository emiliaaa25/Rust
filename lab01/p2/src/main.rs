fn cmmdc(a: i32, b: i32) -> i32 {
    if a * b == 0 {
        return a + b;
    }
    let mut x = a;
    let mut y = b;
    while x != y {
        if x > y {
            x = x - y;
        } else {
            y = y - x;
        }
    }
    return x;
}

fn main() {
    for i in 0..100 {
        for j in 0..100 {
            if cmmdc(i, j) == 1 {
                println!("Numerele {} , {} sunt coprime.", i, j);
            } else {
                println!("Numerele {} , {} nu sunt coprime.", i, j);
            }
        }
    }
}

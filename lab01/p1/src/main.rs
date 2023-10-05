fn prim(n: i32) -> i32 {
    if n < 2 {
        return 0;
    }
    let mut i: i32 = 2;
    while i <= n / 2 {
        if n % i == 0 {
            return 0;
        }
        i = i + 1;
    }
    return 1;
}

fn main() {
    let mut i = 0;
    while i <= 100 {
        if prim(i) == 1 {
            println!("Numarul {} este prim", i);
        }
        if prim(i) == 0 {
            println!("Numarul {} nu este prim", i);
        }
        i = i + 1;
    }
}

fn main() {
    let mut n: i32 = 99;
    while n > 2 {
        println!("{} bottles of beer on the wall,", n);
        println!("{} bottles of beer.", n);
        println!("Take one down, pass it around,");
        n = n - 1;
        println!("{} bottles of beer on the wall.", n);
        println!();
    }

    if n == 2 {
        println!("{} bottles of beer on the wall,", n);
        println!("{} bottles of beer.", n);
        println!("Take one down, pass it around,");
        n = n - 1;
        println!("{} bottle of beer on the wall.", n);
        println!();
    }

    if n == 1 {
        println!("{} bottle of beer on the wall,", n);
        println!("{} bottle of beer.", n);
        println!("Take one down, pass it around,");
        n = n - 1;
        println!("No bottles of beer on the wall.");
        println!();
    }

    if n == 0 {
        println!("No bottles of beer on the wall,");
        println!("No bottles of beer.");
        println!("Go to the store, buy some more,");
        n = 99;
        println!("{} bottles of beer on the wall.", n)
    }
}

fn main() {
    // annotate parsed value
    let guess: u32 = "42".parse().expect("not a number");
    println!("{}", guess);

    println!("{}", 98_222);
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // Destructuring
    let (x, y, z) = tup;

    println!("x {}, y {}, z {}", x, y, z);

    // Dot notation
    let five_hund = tup.0;
    let six_four = tup.1;
    let one = tup.2;

    println!("{}", five_hund);
    println!("{}", six_four);
    println!("{}", one);
}

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);

    x = 6;
    println!("The value of x is: {}", x);

    let spaces = "     ";
    println!("{}", spaces.len());

    // shadowing
    let z = 5;
    let z = z + 1;

    {
        let z = z * 2;
        println!("the value of z in the inner scope is: {}", z);
    }

    println!("the value of z is: {}", z);
}

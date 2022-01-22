use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    println!("Hello, World!");

    // let y = {
    //     let x = 3;
    //     x + 1;
    // };

    // println!("value of y is: {}", y);

    println!("{} of {:b} people know binary, the other half does not.", 1, 2);

    get_time();
    give_me_measurement(3, "hrs");
}

fn get_time() {
    let start = SystemTime::now();
    let since_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("The time went backwards.");

    println!("The time since UNIX_EPOCH: {:?}", since_epoch);
}

fn give_me_measurement(n: i32, unit_label: &str) {
    println!("The measurement is: {}{}", n, unit_label);
}

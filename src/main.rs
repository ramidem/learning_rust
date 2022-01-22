fn main() {
    // LOOP
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);

        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);

            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {}", count);

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {}", result);

    // WHILE
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("Lift off!");

    // FOR
    let collection = [10, 20, 30, 40, 50];
    let mut index = 0;

    // with WHILE
    while index < 5 {
        println!("the value at index {} is: {}", index, collection[index]);

        index += 1;
    }

    println!("===");

    // with FOR
    for (i, item) in collection.iter().enumerate() {
        println!("the value at index {} is: {}", i, item);
    }

    // LIFT OFF with FOR
    for number in (1..11).rev() {
        println!("{}!", number);
    }
    println!("LIFT OFF!!!");
}

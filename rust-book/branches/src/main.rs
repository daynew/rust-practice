fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    loops();
    loop_labels();
    while_loop();
    collection_loop();
}

fn loops() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn loop_labels() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining={remaining}");
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
    println!("End count={count}");
}

fn while_loop() {
    println!("\nwhile_loop");
    let mut number = 3;
    while number != 0 {
        println!("{number}!");

        number -= 1;
    }
}

fn collection_loop() {
    println!("\ncollection_loop");
    let nums = [10, 20, 30, 40, 50];
    for num in nums {
        println!("{num}!");
    }

    println!("\nCountdown starting");
    for num in (1..4).rev() {
        println!("{num}");
    }
    println!("LIFTOFF!!!");
}

fn main() {
    println!("Hello, world!");
    let coin = Coin::Quarter(UsState::Washington);
    match coin {
        Coin::Quarter(UsState::Washington) => {
            println!("The evergreen state");
        }
        coin => {
            println!("It's just a {:?}", coin);
        }
    }
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Washington,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(_state) => 25,
    }
}

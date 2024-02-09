#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

// Count all non-quarter coins we see while also announcing the state of the quarters,
fn main() {
    let mut count = 0;

    let coins = vec![
        Coin::Penny,
        Coin::Nickel,
        Coin::Quarter(UsState::Alabama),
        Coin::Dime,
        Coin::Quarter(UsState::Alaska),
    ];

    for coin in coins {
        if let Coin::Quarter(state) = coin {
            println!("State quarter from {:?}!", state);
        } else {
            count += 1;
        }
    }

    println!("Number of non-quarter coins: {}", count);
}

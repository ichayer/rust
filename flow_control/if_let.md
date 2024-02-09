# Concise Control Flow with if let

The `if let` syntax lets you combine `if` and `let` into a less verbose way 
to handle values that match one pattern while ignoring the rest.

```rust
// A match that only cares about executing code when the value is Some
fn config_max_v1() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
}

// Shorter way using if let
fn config_max_v2() {
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}
```

The syntax `if let` takes a pattern and an expression separated by an equal sign. 
It works the same way as a `match`, where the expression is given to the `match` and the pattern is its first arm.
In this case, the pattern is `Some(max)`, and the `max` binds to the value inside the `Some`. 
We can then use `max` in the body of the `if let` block in the same way we used `max` in the corresponding `match` arm. 
The code in the `if let` block won't run if the value doesn't match the pattern.

Using `if let` means less typing, less indentation, and less boilerplate code. However, you lose the exhaustive checking 
that `match` enforces. Choosing between `match` and `if let` depends on what you’re doing in your particular situation and 
whether gaining conciseness is an appropriate trade-off for losing exhaustive checking.

In other words, you can think of `if let` as syntax sugar for a match that runs code when the value `matches` *one* pattern and 
then ignores all other values.

We can include an `else` with an `if let`. The block of code that goes with the `else` is the same as the block of code that would 
go with the `_` case in the `match` expression.

```rust
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

```

>> If the function only has an effect in one condition, an if let is most idiomatic.
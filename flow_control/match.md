# Match

Rust provides pattern matching via the `match` keyword, which can be used like
a C `switch`. The first matching arm (a pattern to match against) is evaluated and all possible values must be
covered (the compiler checks that all possible cases are handled).

>> If the function needs to return a value for each condition, then a match is most appropriate.

```rust
fn main() {
    let to_guess = 2;
    let my_guess = 3;
    match my_guess.cmp(&to_guess) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!")
    }
}
```


In a `if`, the condition needs to evaluate to a Boolean value, but here it can be any type:

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

// "Coin sorting machine"
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,  // first arm
        Coin::Nickel => 5, // second arm
        Coin::Dime => 10, // ...
        Coin::Quarter => 25
    }
}
```
Think a match as a specialized if that checks for equality of the matched object.

If you want to run multiple lines of code in a match arm, you must use curly brackets, 
and the comma following the arm is then optional.

## Patterns that bind to values

Another useful feature of match arms is that they can bind to the parts of the values that match the pattern. 
This is how we can extract values out of enum variants.

```rust
// As an example, let’s change one of our enum variants to hold data inside it. From 1999 through 2008, 
// the United States minted quarters with different designs for each of the 50 states on one side. 
// No other coins got state designs, so only quarters have this extra value.
// Let’s imagine that a friend is trying to collect all 50 state quarters.

enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

// When we compare that value with each of the match arms, none of them match until we reach Coin::Quarter(state). 
// At that point, the binding for state will be the value UsState::Alaska (whatever). 
// We can then use that binding in the println! expression, thus getting the inner state value out of the 
// Coin enum variant for Quarter.
```

## Catch-all Patterns and the _ Placeholder

Using enums, we can also take special actions for a few particular values, but for all other values take one default action. 

```rust
fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other), // All other cases are covered! "other" will match all values not specifically listed. 
                                     // note that other bind to that value (we are using it).
    }
}

fn add_fancy_hat() {}

fn remove_fancy_hat() {}

fn move_player(num_spaces: u8) {}
```

Rust also has a pattern we can use when we want a catch-all but don’t want to use the value in the catch-all pattern: `_`
is a special pattern that matches any value and does not bind to that value. This tells Rust we aren’t going to use the value, 
so Rust won’t warn us about an unused variable.

```rust
fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }
}

fn add_fancy_hat() {}

fn remove_fancy_hat() {}

fn reroll() {}
```

We’ll change the rules of the game one more time so that nothing else happens on your turn if you roll anything 
other than a 3 or a 7:

```rust
fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
}
```

Here, we’re telling Rust explicitly that we aren’t going to use any other value that doesn't match a pattern in an earlier arm, 
and we don’t want to run any code in this case.

## Match and Ownership

If an enum contains non-copyable data like a `String`, 
then you should be careful with whether a match will move or borrow that data. 

<div style="text-align:center">

![matches - 1.png](..%2Fimages%2Fmatches%20-%201.png)

</div>

But if we replace the placeholder in `Some(_)` with a variable name, like `Some(s)`, then the program will NOT compile

<div style="text-align:center">

![matches - 2.png](..%2Fimages%2Fmatches%20-%202.png)

</div>

`opt` is a plain enum — its type is `Option<String>` and not a reference like `&Option<String>`. 
Therefore, a match on `opt` will move non-ignored fields like `s`. Notice how `opt` loses read and own 
permission sooner in the second program compared to the first. After the match expression, the data within `opt`
has been moved, so it is illegal to read `opt` in the `println`.

If we want to peek into `opt` without moving its contents, the idiomatic solution is to match on a reference:

<div style="text-align:center">

![matches - 3.png](..%2Fimages%2Fmatches%20-%203.png)
</div>

Rust will “push down” the reference from the outer enum, `&Option<String>`, to the inner field, `&String`. 
Therefore, `s` has type `&String`, and `opt` can be used after the match.
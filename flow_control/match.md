# Match

Rust provides pattern matching via the `match` keyword, which can be used like
a C `switch`. The first matching arm (a pattern to match against) is evaluated and all possible values must be
covered.

```rust
match myguess.cmp(&toguess) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
```
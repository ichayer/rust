# Recoverable errors with `Result`

Most errors aren’t serious enough to require the program to stop entirely. Sometimes, when a function fails, 
it’s for a reason that you can easily interpret and respond to.

For example, if you try to open a file and that operation fails because the file doesn't exist, you might want to create the 
file instead of terminating the process.


We've already covered the basics about Result enum here: [result.md](..%2Fstd_library%2Fresult.md). Let's delve a bit deeper.

```rust
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}
```

The code above will `panic!` no matter why `File::open` failed. However, we want to take different actions 
for different failure reasons.

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() { // io::Error
            // file doesn't exist, create it and return the handle to it
            ErrorKind::NotFound => match File::create("hello.txt") {  // File::create could also fail
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}
```

That’s a lot of `match`! The `match` expression is very useful but also very much a primitive.
In the future, we will learn about closures, which are used with many of the methods defined on `Result`.
These methods can be more concise than using match when handling `Result` values in your code. 

For example, here’s another way to write the same logic as shown above, this time using closures and the `unwrap_or_else` method:

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
```

Many more of these methods can clean up huge nested `match` expressions when you’re dealing with errors.

## Shorcuts for Panic on Error: `unwrap` and `expect`

The `Result<T, E>` type has many helper methods defined on it to do various, more specific tasks. 
The `unwrap` method is a shortcut method implemented just like the `match` expression. If the `Result` value is the 
`Ok` variant, `unwrap` will return the value inside the `Ok`. If the `Result` is the `Err` variant, `unwrap` will call 
the `panic!` macro for us. Here is an example of `unwrap` in action:

```rust
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap();
}
```

If we run this code without a `hello.txt` file, we’ll see an error message from the `panic!` call that the `unwrap` method makes.

Similarly, the `expect` method lets us also choose the `panic!` error message. Using `expect` instead of `unwrap` and providing
good error messages can convey your intent and make tracking down the source of a panic easier.

```rust
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}
```

In production-quality code, most Rustaceans choose `expect` rather than `unwrap` and give more context about why the 
operation is expected to always succeed. That way, if your assumptions are ever proven wrong, you have more information \
to use in debugging.
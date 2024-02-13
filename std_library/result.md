# [std::result::Result](https://doc.rust-lang.org/std/result/index.html)

When we want to express why an operation failed, we have the `Result` enum. Functions return `Result` 
whenever errors are expected and recoverable. In the `std` crate, `Result` is most prominently used for `I/O`.

Result enum and its variants will be brought into scope by the prelude.

>> Functions usually return a Result and not an Option because Option can just represent that an operation has failed, 
but Result can explain why the operation has failed. For example, file opening can fail for many reasons.

```rust
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

- `Ok(value)` indicates that the operation succeeded, and wraps the `value` returned by the operation
- `Err(why)` indicates that the operation failed, and wraps `why`, which (hopefully) explains the cause of the failure

A common problem with using return values to indicate errors is that it is easy to ignore the return value, 
thus failing to handle the error. `Result` is annotated with the #[must_use] attribute, which will cause 
the compiler to issue a warning when a `Result` value is ignored. This makes `Result` especially useful with 
functions that may encounter errors but don’t otherwise return a useful value.

You might instead, if you don’t want to handle the error, simply assert success with `expect`. This will panic 
if the write fails, providing a marginally useful message indicating why or propagate the error up the call stack 
with `?`. Ending the expression with `?` will result in the `Ok’s` unwrapped value, unless the result is `Err`, in 
which case `Err` is returned early from the function as if we had used the `return` keyword.
We’re only allowed to use the `?` operator in a function that returns `Result`, `Option`, or another type that 
implements `FromResidual`.

```rust
// Representation of ? as code
fn main() {
    // e of type Result<T,E>
    match e {
        Ok(x) => x,
        Err(err) => { return Err(err); }
    }
}
```

```rust
// Function to perform division and return a Result
fn divide_ints(a: i32, b: i32) -> Result<i32, &'static str> {
    if b == 0 {
        Err("Cannot divide by zero!")
    } else {
        Ok(a / b)
    }
}

// Function that calls divide and uses ? to propagate error
fn divide_and_print(a: i32, b: i32) -> Result<(), &'static str> {
    let result = divide_ints(a, b)?;
    println!("Result: {}", result);
    Ok(())
}

fn main() {
    
    // Using ? to propagate errors and printing a success message
    if let Err(e) = divide_and_print(10, 0) {
        eprintln!("Error: {}", e);
    }

    // Using expect to assert success and panic (if appropiate) on failure
    divide_and_print(10, 0).expect("Failed to divide and print!");
}
```

```rust
// Using the ? operator on an Option<T> value
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
```

Watch out for whether you are using `?` in a function that returns an `Option` or a `Result`, and ensure it aligns with your 
function's return type. The `?` operator won’t automatically convert a `Result` to an `Option` or vice versa; in those cases, 
you can use methods like the `ok` method on `Result` or the `ok_or` method on `Option` to do the conversion explicitly.
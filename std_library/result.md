# [std::result::Result](https://doc.rust-lang.org/std/result/index.html)

When we want to express why an operation failed, we have the `Result` enum. Functions return `Result` whenever errors are expected and recoverable. In the `std` crate, `Result` is most prominently used for `I/O`.

```rust
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

- `Ok(value)` indicates that the operation succeeded, and wraps the `value` returned by the operation
- `Err(why)` indicates that the operation failed, and wraps `why`, which (hopefully) explains the cause of the failure

A common problem with using return values to indicate errors is that it is easy to ignore the return value, thus failing to handle the error. `Result` is annotated with the #[must_use] attribute, which will cause the compiler to issue a warning when a `Result` value is ignored. This makes `Result` especially useful with functions that may encounter errors but don’t otherwise return a useful value.

You might instead, if you don’t want to handle the error, simply assert success with `expect`. This will panic if the write fails, providing a marginally useful message indicating why or propagate the error up the call stack with `?`. Ending the expression with `?` will result in the `Ok’s` unwrapped value, unless the result is `Err`, in which case `Err` is returned early from the enclosing function.

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

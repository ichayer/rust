# Unrecoverable Errors with `panic!`

Sometimes, bad things happen in your code, and there’s nothing you can do about it. In these cases, 
Rust has the `panic!` macro. There are two ways to cause a panic in practice: by taking an action that 
causes our code to panic (such as accessing an array past the end) or by explicitly calling the `panic!` macro.

A `panic!` should not be used to communicate failure within the program. The default assumption is that caller functions 
will not try to catch panics.

## Unwinding the stack or aborting in response to a panic

By default, when a panic occurs, the program starts unwinding, which means Rust walks back up the 
stack and cleans up the data from each function it encounters. However, this walking back and cleanup is a 
lot of work. Rust, therefore, allows you to choose the alternative of immediately aborting, which ends the 
program without cleaning up. When aborting, memory that the program was using will then need to be cleaned 
up by the operating system.

```console
// cargo.toml
[profile.release]
panic = 'abort'
```

## Using a `panic!` backtrace

Let’s look at another example to see what it’s like when a `panic!` call comes from a library because of a bug 
in our code instead of from our code calling the macro directly. 

```rust
fn main() {
    let v = vec![1, 2, 3];
    v[99];
}
```

If we run this code, program will panic and will throw the following output:

```console
index out of bounds: the len is 3 but the index is 99
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

A backtrace is a list of all the functions that have been called to get to this point. Backtraces in Rust 
work as they do in other languages: the key to reading the backtrace is to start from the top and read until 
you see files you wrote. That’s the spot where the problem originated. The lines above that spot are code 
that your code has called; the lines below are code that called your code. These before-and-after lines might 
include core Rust code, standard library code, or crates that you’re using. 

```console
$env:RUST_BACKTRACE=1; cargo run # windows
```


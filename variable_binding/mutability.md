# Mutability

Variable bindings are immutable by default (once we give the variable a value, the value won’t change), but this can be overridden using the `mut` modifier.

```rust
fn main() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    // Ok
    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    // Error! Cannot assign a new value to an immutable variable
    _immutable_binding += 1;
}
```

The compiler will throw a detailed diagnostic about mutability errors.

Let’s explore how and why Rust encourages you to favor immutability and why sometimes you might want to opt out:

> It’s important that we get compile-time errors when we attempt to change a value that’s designated as immutable because this very situation can lead to bugs. If one part of our code operates on the assumption that a value will never change and another part of our code changes that value, it’s possible that the first part of the code won’t do what it was designed to do. The cause of this kind of bug can be difficult to track down after the fact, especially when the second piece of code changes the value only sometimes. The Rust compiler guarantees that when you state that a value won’t change, it really won’t change, so you don’t have to keep track of it yourself. Your code is thus easier to reason through. Ultimately, deciding whether to use mutability or not is up to you and depends on what you think is clearest in that particular situation.
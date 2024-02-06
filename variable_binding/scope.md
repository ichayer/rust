# Variable scope and shadowing

Variable bindings have a scope, and are constrained to live in a `block`. A
block is a collection of statements enclosed by braces `{}`.
```rust
fn main() {
    // This binding lives in the main function
    let main_variable_binding = 1;

    // This is a block, and has a smaller scope than the main function
    {
        // This binding only exists in this block
        let block_variable_binding = 2;
        
        // Ok
        println!("{}", block_variable_binding);
    }

    // Error! `short_lived_binding` doesn't exist in this scope
    println!("{}", block_variable_binding);
    
    // Ok
    println!("{}", main_variable_binding);
}

```
Also, shadowing is allowed.
```rust
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
```

Shadowing is different from marking a variable as `mut` because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the `let` keyword. By using `let`, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.

The other difference between `mut` and shadowing is that because we’re effectively creating a new variable when we use the `let` keyword again, we can change the type of the value but reuse the same name.

```rust
let spaces = "   "; 
let spaces = spaces.len();
```
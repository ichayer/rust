# Functions

Functions are declared using the `fn` keyword. 

Rust code uses snake case as the conventional style for function and variable 
names, in which all letters are lowercase and underscores separate words.

```rust
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

## Parameters

Its **arguments** (or parameters) are type 
annotated, just like variables.

```rust
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
```

## Statements and Expressions

### Statements
Statements are instructions that perform some action and do not return a value. 
Rust has two kinds of statement: declaration statements and expression statements.

### Expressions
Expressions evaluate to a resultant value.

## Functions with return values
If the function returns a value, the return type must be specified after an arrow `->`.

Function bodies are made up of a series of statements optionally ending in an expression.
This final expression in the function will be used as return value.
Alternatively, the `return` statement can be used to return a value earlier
from within the function, even from inside loops or `if` statements.

```rust
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
```
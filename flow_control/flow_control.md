# Flow of Control

An integral part of any programming language are ways to modify control flow.

## If / else if / else expressions
An `if` expression allows you to branch your code depending on conditions. 
You must be explicit and always provide if with a boolean as its condition.

>> Using too many else if expressions can clutter your code, so if you have more than one, 
you might want to refactor your code. Rust has a powerful branching construct called `match` for these cases.

Because `if` is an expression, we can use it on the right side of a let statement to assign the outcome to a variable:

```rust
fn main() {
    let condition = true;
    
    // The expression in the if/else block must evaluate to the same data type
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}
```

## Loops

### loop
The `loop` keyword tells Rust to execute a block of code over and over again forever or until you explicitly tell it to stop.

One of the uses of a `loop` is to retry an operation you know might fail, such as checking whether a thread has completed its job. 
You might also need to pass the result of that operation out of the loop to the rest of your code. To do this, 
you can add the value you want returned after a `break` expression you use to stop the loop; that value will be 
returned out of the loop so you can use it.

```rust
fn main() {
    let mut counter = 0;
    
    // hold the value returned from the loop
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
```

If you have loops within loops, `break` and `continue` apply to the innermost loop at that point. 
You can optionally specify a `loop label` on a loop that you can then use with `break` or `continue`
to specify that those keywords apply to the labeled loop instead of the innermost loop. 
Loop labels must begin with a single quote. Here’s an example with two nested loops:

```rust
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
```

### while
A program will often need to evaluate a condition within a loop. While the condition is true, the loop runs. 
When the condition ceases to be true, the program calls break, stopping the loop. In this cases we use `while`.

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
```
This construct eliminates a lot of nesting that would be necessary if you used loop, if, else, and break, and it’s clearer. 
While a condition evaluates to true, the code runs; otherwise, it exits the loop.

### for

We use `for` as a more concise alternative for looping and executing some code for each item in a collection.

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
```

This code with `for` is safe because we eliminated the chance of bugs that might result
from going beyond the end of the array or not going far enough and missing some items if we used the `while` approach:

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}
```

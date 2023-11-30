# References

```rust
 io::stdin()
        .read_line(&mut input)
```

The `&` indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times. References are a complex feature, and one of Rust’s major advantages is how safe and easy it is to use references. 

For now, all we know is that, like variables, references are immutable by default. Hence, we will write `&mut input` rather than `&input` to make it mutable.
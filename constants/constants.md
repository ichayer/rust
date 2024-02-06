# Constants

Constants can be declared within any scope, including the global scope, and require explicit data type annotations
They may be set only to a constant expression, not the result of a value that could only be [computed at runtime](https://doc.rust-lang.org/reference/const_eval.html).

Rust adheres to a naming convention for constants, employing all uppercase letters with underscores between words.

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```


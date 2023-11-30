# [std::cmp::Ordering](https://doc.rust-lang.org/std/cmp/enum.Ordering.html)

An `Ordering` is the result of a comparison between two values.

```rust
pub enum Ordering {
    Less,
    Equal,
    Greater,
}
```

- Less: An ordering where a compared value is less than another
- Equal: An ordering where a compared value is equal to another
- Greater: An ordering where a compared value is greater than another
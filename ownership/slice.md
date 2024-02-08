# The Slice Type

Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection. 
A slice is a kind of reference, so it does not have ownership.

## String slice

For the purposes of introducing string slices, we are assuming ASCII only in this section.

A string slice is a reference to part of a String, and it looks like this:

```rust
fn main() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
}
```

Internally, the slice data structure stores the starting position and the length of the slice, 
which corresponds to ending_index minus starting_index. So, in the case of let world = &s[6..11];, 
world would be a slice that contains a pointer to the byte at index 6 of s with a length value of 5.

## String literals are slices

Recall that we talked about string literals being stored inside the binary. Now that we know about slices, 
we can properly understand string literals:

```rust
let s = "Hello, world!";
```

The type of s here is `&str`: it’s a slice pointing to that specific point of the binary. 
This is also why string literals are immutable; &str is an immutable reference.

## Other slices 

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3])
}
```
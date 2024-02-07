# Primitives

Every value in Rust is of a certain data type, which tells Rust what kind of data is being specified 
so it knows how to work with that data. Rust is a statically typed language, 
which means that it must know the types of all variables at compile time.

## Scalar Types

### Integer

<div style="margin-left: auto;
            margin-right: auto;
            width: 30%">

| Length  |        Signed        | Unsigned |
|:-------:|:--------------------:|:--------:|
|  8-bit  |         `i8`         |   `u8`   |
| 16-bit  |        `i16`         |  `u16`   |
| 32-bit  | `i32` (rust default) |  `u32`   |
| 64-bit  |        `i64`         |  `u64`   |
| 128-bit |        `i128`        |  `u128`  |
|  arch   |       `isize`        | `usize`  |


</div>

The `isize` and `usize` types depend on the architecture of the computer your program is running on, which is 
denoted in the table as `arch` (pointer size): 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 
32-bit architecture. The primary situation in which you’d use them is when indexing some sort of collection.

>> Relying on integer overflow’s wrapping behavior is considered an error. To explicitly handle the possibility of overflow, you can use families of methods provided by the standard library for primitive numeric types.

### Floating point
* `f32` or `f64` (rust default)
* Always signed

### Boolean type
* `bool` either `true` or `false`

### Char type
* `char` Unicode scalar values like `'a'`, `'α'` and `'∞'` (4 bytes each).

>> Note that we specify char literals with single quotes, as opposed to string literals, which use double quotes. Rust’s char type is four bytes in size and represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII

## Compound Types

### Arrays
* Unlike arrays in some other languages, arrays in Rust have a fixed length
* Every element of an array must have the same type
* 

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```

```rust
// How to access elements using indexing
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
```

>> Arrays are useful when you know the number of elements will not need to change. If so, use Vector.

>> Arrays are useful when you want your data allocated on the stack rather than the heap

>> When you attempt to access an element using indexing, Rust will check that the index you’ve specified is less than 
the array length. If the index is greater than or equal to the length, Rust will panic. 
This check happens at runtime. This is an example of Rust’s memory safety principles in action. 
In many low-level languages (like C), this kind of check is not done, and when you provide an incorrect index, 
invalid memory can be accessed. Rust protects you against this kind of error by immediately exiting instead 
of allowing the memory access and continuing.

### Tuples

* Tuples have a fixed length: once declared, they cannot grow or shrink in size
* Element of a tuple can have different type
* The tuple without any values has a special name, `unit`. This value and its corresponding type are both written `()` 
and __represent an empty value or an empty return type__ 
* Expressions implicitly return the unit value if they don’t return any other value

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

```rust
// Destructuring a tuple in its components
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
}
```

```rust
// Access tuple component with a '.'
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
```

# Explicit and implicit variable type annotation

Variables can always be *type annotated*. Numbers may additionally be annotated
via a *suffix* or *by default*. Integers default to `i32` and floats to `f64`.
Note that Rust can also infer types from context.

```rust
fn main() {
    // Variables can be type annotated.
    let logical: bool = true;

    let a_float: f64 = 1.0;  // Regular annotation
    let an_integer   = 5i32; // Suffix annotation
    
    let default_float   = 3.0; // `f64`
    let default_integer = 7;   // `i32`

    // A type can also be inferred from context.
    let mut inferred_type = 12; // Type i64 is inferred from another line.
    inferred_type = 3294967296i64;
}
```
# Vector [`Vec<T>`](https://doc.rust-lang.org/std/vec/struct.Vec.html)

* Vectors allow you to store more than one value in a single data structure that puts all the values next to each other in the heap. 
* Vectors can only store values of the same type.
* Vectors are indexed by number, starting at zero

## Creating an empty vector
```rust
fn main() {
    let v: Vec<i32> = Vec::new(); // Rust knows that the Vec<T> in v will hold elements of the i32 type.
    let v1 = Vec::new(); // Also valid
}
```

## Creating a new vector with elements
More often, you’ll create a `Vec<T>` with initial values and Rust will infer the type of value you want to store.
Rust conveniently provides the `vec!` macro, which will create a new vector that holds the values you give it.

```rust
fn main() {
    let v = vec![1, 2, 3];
}
```

The integer type is `i32` because that’s the default integer type, then, `Rust` can infer that the type of `v` is `Vec<i32>`.

## Updating a vector

As with any variable, if we want to be able to change its value, we need to make it mutable using the `mut` keyword.

```rust
fn main() {
    // Adding elements to an empty vector
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
}
```

Warning: `Vec::push` will move its argument unless it can be copies (copy trait)

```rust
fn main() {
    let mut v = Vec::new();
    let s = String::from("Hello ");
    v.push(s);  // s is not usable after calling v.push(s)
    println!("original: {}", s); 
    println!("new: {}", v[0]);
}
```

## Reading elements of vectors

There are two ways to reference a value stored in a vector: via indexing or using the get method. 

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2]; // reference to the element
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2); // reference to the element
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
}
```

When the program has a valid reference, the borrow checker enforces the ownership and borrowing rules 
to ensure this reference and any other references to the contents of the vector remain valid. Recall the rule 
that states you **can’t** have mutable and immutable references in the same scope. 

```rust
fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0]; // immutable borrow occurs here
    v.push(6); // mutable borrow occurs here, error!
    println!("The first element is: {first}");
}
```

Why should a reference to the first element care about changes at the end of the vector? due to the way vectors work. 

Because vectors put the values next to each other in memory, adding a new element onto the end of the vector might 
require allocating new memory and copying the old elements to the new space, if there isn’t enough room to put all the 
elements next to each other where the vector is currently stored. In that case, the reference to the first element would be 
pointing to deallocated memory. The borrowing rules prevent programs from ending up in that situation.

Recall that non-copyable types cannot be moved out of a vector by indexing:
```rust
fn main() {
  let v = vec![String::from("Hello ")];
  let mut s = v[0]; // Error!
  s.push_str("world");
  println!("{s}");
}
```
If a value does not own heap data, then it can be copied without a move. A String does own heap data, so it can not
be copied without a move.

So if we have a vector of non-Copy types like String, then how do we safely get access to an element of the vector? 
Here's a few different ways to safely do so. First, you can avoid taking ownership of the string and just use an 
immutable reference:

```rust
fn main() {
    let v = vec![String::from("Hello ")];
    let s = &v[0];
    println!("{s}");
}
```

Second, you can clone the data if you want to get ownership of the string while leaving the vector alone:

```rust
fn main() {
    let v: Vec<String> = vec![String::from("Hello world")];
    let mut s: String = v[0].clone();
    s.push('!');
    println!("{s}");
}
```

Finally, you can use a method like `Vec::remove` to move the string out of the vector:

```rust
fn main() {
    let mut v: Vec<String> = vec![String::from("Hello world")];
    let mut s: String = v.remove(0);
    s.push('!');
    println!("{s}");
    assert!(v.len() == 0);
}
```

### Tricky example

Determine whether the program will pass the compiler:
```rust

fn main() {
  let mut v: Vec<i32> = vec![1, 2, 3];
  let mut v2: Vec<&mut i32> = Vec::new();
  for i in &mut v {
    v2.push(i);
  }
  *v2[0] = 5;
  let a = *v2[0];
  let b = v[0];
  println!("{a} {b}");
}
```

The program does compile and its output is 5 5.
`i` has type `&mut i32`, meaning it is a pointer to a number within `v`. So if we push `i` into `v2`, then `v2` contains pointers to `v`. 
Therefore mutating `v2[0]` actually mutates `v[0]`.

## Iterating over the values in a vector

To access each element in a vector in turn, we would iterate through all the elements 
rather than use indices to access one at a time using a `for` loop.

```rust
fn main() {
    let v = vec![100, 32, 57];
    for n_ref in &v { // get immutable &i32 references of each element in v
        let n_plus_one: i32 = *n_ref + 1;
        println!("{n_plus_one}");
    }
}
```

```rust
fn main() {
    let mut v = vec![100, 32, 57];
    for n_ref in &mut v { // get mutable &i32 references of each element in v
        *n_ref += 50;
    }
}
```

### Safely using iterators

Iterators contain a pointer to data within the vector.

```rust
fn dup_in_place(v: &mut Vec<i32>) {
    for n_ref in v.iter() {
        v.push(*n_ref);
    }
}
```

<div style="text-align:center">

![vector - 1.png](..%2Fimages%2Fvector%20-%201.png)

</div>

Notice that `v.iter()` removes the `W` permission from `*v`. Therefore the `v.push(..)` operation is missing the 
expected `W` permission. 

The safety issue beneath this error is reading deallocated memory. As soon as `v.push(...)` happens, the vector will 
reallocate its contents and invalidate the iterator's pointer. So to use iterators safely, Rust does not allow you to 
add or remove elements from the vector during iteration.

<div style="text-align:center">

![vector - 2.png](..%2Fimages%2Fvector%20-%202.png)

</div>

## Using an enum to store multiple types

Vectors can only store values that are the same type. This can be inconvenient; there are definitely 
use cases for needing to store a list of items of different types. Fortunately, the variants of an enum are 
defined under the same enum type, so when we need one type to represent elements of different types, we can define and use 
an enum!

```rust
  enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
```

## Dropping a Vector drops its elements

Like any other struct, a vector is freed when it goes out of scope.

```rust
fn main() {
    let v = vec![1, 2, 3, 4];

    // do stuff with v
} // <- v goes out of scope and is freed here
```

When the vector gets dropped, all of its contents are also dropped, meaning the integers it holds will be cleaned up. 
The borrow checker ensures that any references to contents of a vector are only used while the vector itself is valid.
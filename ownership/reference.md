# References

Taking ownership and then returning ownership with every function is a bit tedious. What if we want to let
a function use a value but not take ownership? It’s quite annoying that anything we pass in also needs to
be passed back if we want to use it again, in addition to any data resulting from the body of the function
that we might want to return as well.

Rust has a feature for using a value without transferring ownership, called references.

A reference is like a pointer in that it’s an address we can follow to access the data stored at that address; 
that data is owned by some other variable. Unlike a pointer, a reference is guaranteed to point to 
a valid value of a particular type for the life of that reference.

```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()  // Here, s goes out of scope. But because it does not have ownership of what
             // it refers to, it is not dropped.
}
```

The `&s1` syntax lets us create a reference that refers to the value of s1 but does not own it. 
Because it does not own it, the value it points to will not be dropped when the reference stops being used.

We call the action of creating a reference **borrowing**. 

## Mutable references

Just as variables are immutable by default, so are references.

```rust
fn main() {
    let s = String::from("hello");
    change(&s); // Error!
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}
```

Can we fix the code above to be able to modify a borrowed value? Yes!

```rust
fn main() {
    let mut s = String::from("hello"); // Now s is mut
    change(&mut s); // Create a mutable reference with &mut s
}

fn change(some_string: &mut String) { // This makes it very clear that the 'change' function will mutate the value it borrows.
    some_string.push_str(", world");
}
```

Mutable references have one big restriction: if you have a mutable reference to a value in a scope, 
you can have no other references to that value in the same scope. The benefit of having this restriction is that Rust can prevent 
data races at compile time. A data race is similar to a race condition and happens when these three behaviors occur:

* Two or more pointers access the same data at the same time.
* At least one of the pointers is being used to write to the data.
* There’s no mechanism being used to synchronize access to the data.

Data races cause undefined behavior and can be difficult to diagnose and fix when you’re trying to track them down at runtime; 
Rust prevents this problem by refusing to compile code with data races!

Rust enforces a similar rule for combining mutable and immutable references.

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem 
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);
    
    // cannot borrow `s` as mutable because it is also borrowed as immutable!
    // Multiple immutable references are allowed because they are just "read only pointers"
}
```

## Reference Scope
Reference’s scope starts from where it is introduced and continues through the last time that reference is used. 

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}
```

## Dangling reference

In languages with pointers, it’s easy to erroneously create a dangling pointer—a pointer that references a location 
in memory that may have been given to someone else—by freeing some memory while preserving a pointer to that memory.

```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String { // dangle returns a reference to a String
    let s = String::from("hello"); // s is a new String
    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
```

Because s is created inside dangle, when the code of dangle is finished, s will be deallocated. 
But we tried to return a reference to it. That means this reference would be pointing to an invalid String. 
That’s no good! Rust won’t let us do this. The solution for this is transfer ownership.
# Ownership

Ownership is a set of rules that govern how a Rust program manages memory while running.

Some languages have garbage collection (Java) that regularly looks for no-longer-used memory as the program runs; 
in other languages, the programmer must explicitly allocate and free the memory (C).
Rust uses a third approach: memory is managed through a system of ownership with a set of rules that the compiler checks. 
If any of the rules are violated, the program won’t compile. None of the features of ownership will slow down your program while it’s running.

## Stack vs Heap

Both of them are parts of memory available to your code to use at runtime, but they are structured in different ways.

* The stack stores values in the order it gets them and removes the values in the opposite order. 
This is referred to as last in, first out. Adding data is called pushing onto the stack, and removing data is called 
popping off the stack. All data stored on the stack must have a known, fixed size.
* Data with an unknown size at compile time or a size that might change must be stored on the heap instead.
The heap is less organized: when you put data on the heap, you request a certain amount of space. The memory allocator 
finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of 
that location. This process is called allocating on the heap and is sometimes abbreviated as just allocating. 
Because the pointer to the heap is a known, fixed size, you can store the pointer on the stack, but when you want the actual
data, you must follow the pointer.

### Comparison
* Pushing to the stack is faster than allocating on the heap because the allocator never has to search for a place to store new data; that location is always at the top of the stack.
* Accessing data in the heap is slower than accessing data on the stack because you have to follow a pointer to get there

### Key point
Keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap, 
and cleaning up unused data on the heap so you don’t run out of space are all problems that ownership addresses. 

## Rules
* Each value in Rust has an owner.
* There can only be one owner at a time.
* When the owner goes out of scope, the value will be dropped.

## Rules explanation

We've explored the storage of [primitives](../primitives.md) in the stack due to their known size. 
Additionally, we've delved into the [scope of variables](../variable_binding/scope.md). Let's try to understand
now how ownership is established for variables stored in the heap, considering their scope.
We’ll concentrate on the parts of String that relate to ownership. 
These aspects also apply to other complex data types, whether they are provided by the standard library or created by you.

* String literals -> Immutable and known text at compile -> stack
* String -> Mutable and unknown text at compile time (user input) -> heap

This means that for a String we need:
* To request memory from the memory allocator at runtime (already done when we use String::from)
* A way of returning this memory to the allocator when we’re done with our String

Managing memory deallocation in C was challenging since we had to remember and to meticulously pair each 
allocation with its corresponding free. In contrast, in Java, we benefit from the Garbage Collector (GC).
Rust takes a different path: the memory is automatically returned once the variable that owns it goes out of scope.

```rust
fn main() {
    {
        let s = String::from("hello"); // s is valid from this point forward
        // do stuff with s
    } // this scope is now over, and s is no longer valid
}
```
When a variable goes out of scope, Rust calls `drop` automatically and cleans up
the heap memory for that variable. Nice!

### Double free error to explain second rule (move)

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
}
```

A String consist of a pointer, its length and the capacity. When we assign s1 to s2, the String data is copied, 
meaning we copy the pointer, the length, and the capacity that are on the stack. 
We do not copy the data on the heap that the pointer refers to.

Earlier, we said that when a variable goes out of scope, Rust automatically calls the drop function and cleans up 
the heap memory for that variable. But from C we now that both data pointers are pointing to the same location. 
This is a problem: when s2 and s1 go out of scope, they will both try to free the same memory. This is known as a double 
free error and is one of the most common memory safety bugs. Freeing memory twice can lead to memory corruption, 
which can potentially lead to security vulnerabilities.

To ensure memory safety, after the line let s2 = s1;, Rust considers s1 as no longer valid. 
Therefore, Rust doesn't need to free anything when s1 goes out of scope. 

If you’ve heard the terms shallow copy and deep copy while working with other languages, the concept of copying the pointer, 
length, and capacity without copying the data probably sounds like making a shallow copy. 
But because Rust also invalidates the first variable, instead of being called a shallow copy, it’s known as a **move**. 
In this example, we would say that s1 was **moved** into s2.

If we do want to deeply copy the heap data and not only the stack data, we can use clone (expensive).

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}
```

>> Rust has a special annotation called the Copy trait that we can place on types that are stored on the stack, 
as integers are. If a type implements the Copy trait, variables that use it do not move, but rather are trivially copied, 
making them still valid after assignment to another variable unless it has implemented the Drop trait.

### Ownership - Functions
The mechanics of passing a value to a function are similar to those when assigning a value to a variable. Passing a variable to a function will move or copy, 
just as assignment does

```rust
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens. If we tried to use s after the call to takes_ownership, Rust would throw a compile-time error.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
```

### Ownership - Return values and scope
Returning values can also transfer ownership

```rust
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}
```
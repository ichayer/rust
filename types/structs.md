# Structs

A struct, or structure, is a custom data type that lets you package together 
and name multiple related values that make up a meaningful group. 

## Structs vs Tuples
* Both hold multiple related values of different types
* Struct has named fields: you don’t have to rely on the order of the data to specify or access the values of an instance.

## Usage

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    }; // instance
}
```
<div style="text-align:center">

![structs - 1.png](..%2Fimages%2Fstructs%20-%201.png)

</div>

To get a specific value from a struct, we use dot notation. If the instance is mutable, 
we can change a value by using the dot notation and assigning into a particular field.
Note that the entire instance must be mutable or not; Rust doesn't allow us to mark only certain fields as mutable.

## Field Init Shorthand

```rust
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
```

Given that the username and email fields share the same name as the corresponding parameters (username and email), 
there is no need to redundantly specify both, simplifying the expression to just "username" and "email" instead of 
"email: email" or "username: username."

## Creating Instances from Other Instances with Struct Update Syntax

It’s often useful to create a new instance of a struct that includes most of the values from another instance, 
but changes some. You can do this using struct update syntax.

```rust
fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    
    // Don't do this!
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    
    // Use struct update syntax instead
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 // Any remaining fields should get their values from the corresponding fields in user1
    };
}
```

In this example, we can no longer use user1 after creating user2 because the String in the username field of 
user1 was moved into user2. If we had given user2 new String values for both email and username, 
and thus only used the active and sign_in_count values from user1, 
then user1 would still be valid after creating user2. The types of active and sign_in_count are types 
that implement the Copy trait

## Tuple structs

Tuple structs have the added meaning the struct name provides but don’t have names associated with their fields; 
rather, they just have the types of the fields.

Tuple structs are useful when you want to give the whole tuple a name and make the tuple a different type from other 
tuples, and when naming each field as in a regular struct would be verbose or redundant.

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
```

Each struct you define is its own type, even though the fields within the struct might have the same types.

## Unit-like structs

Type of struct that don’t have any fields.

Unit-like structs can be useful when you need to implement a trait on some type but don’t have 
any data that you want to store in the type itself

```rust
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```

## Borrowing fields of a struct
Rust's borrow checker will track ownership permissions at both the struct-level and field-level. 

<div style="text-align:center">

![structs - 2.png](..%2Fimages%2Fstructs%20-%202.png)
</div>

<div style="text-align:center">

![structs - 3.png](..%2Fimages%2Fstructs%20-%203.png)
</div>

```rust

struct Point {
  x: i32,
  y: i32,
}
fn main() {
  let mut p = Point { x: 1, y: 2 };
  let x = &mut p.x;   // Rust understands that .x refers to a different object 
  let y = &mut p.y;   // than .y, so it is valid to take simultaneous mutable references to both fields.
  *x += 1;
  *y += 1;
  println!("{} {}", p.x, p.y);
}
```

# Enums

Enums allow you to define a type by enumerating its possible variants.

>> Enums can provide a clearer representation of mutually exclusive states, making it easier to express and enforce the desired
conditions in a data structure.

It's really useful using the [match](..%2Fflow_control%2Fmatch.md) control flow construct with enums.

```rust
enum IpAddrKind {
    V4,
    V6,
}
```

Any IP address can be either a version four or a version six address, but not both at the same time. 
That property of IP addresses makes the enum data structure appropriate because an enum value can only be one of its variants.

`IpAddrKind` is now a custom data type that we can use elsewhere in our code.

```rust
fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
}
```

Because both options are of the same type (`IpAddrKind`) we can define a function that takes any `IpAddrKind`.

```rust
fn route(ip_kind: IpAddrKind) {
    // function body
}
```

>> Defining a function for an enum that holds different kinds of elements is easier than having several structs. 
All the elements of an enum are of the same type (the enum type), while structs have their own independent type

Rust enums can hold data:

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
}
```

Rust enums can define methods:

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IpAddr {
    fn route(&self) {
        // method body
    }
}
```

## The Option Enum and Its Advantages Over Null Values

*Rust does not have null pointers*, so the `null` keyword does not exist. So, how do we handle the scenario of a value that is currently invalid or absent for some reason? 

With the Option type. `Option` is one enum defined by the standard library:

```rust
enum Option<T> {
    None,
    Some(T),
}
```

It's included in the Prelude. So you don't need to bring it into the scope explicitly.


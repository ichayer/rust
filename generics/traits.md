# Traits: Defining Shared Behavior

A `trait` defines functionality a particular type has and can share with other types. We can use `traits` to define 
shared behavior in an abstract way. We can use `trait bounds` to specify that a **generic type** can be any type 
that has certain behavior.

>> Traits are similar to Java interfaces, although with some differences.

## Defining a Trait and implementing it on a type

A type’s behavior consists of the methods we can call on that type. 
Different types share the same behavior if we can call the same methods on all of those types. 
Trait definitions are a way to group method signatures together to define a set of behaviors necessary to accomplish some purpose.

We want to make a media aggregator **library** crate named `aggregator` that can display summaries of data that might be 
stored in a `NewsArticle` or `Tweet` instance. To do this, we need a summary from each type, and we’ll request that summary 
by calling a `summarize` method on an instance. The code below shows the definition of a **public** `Summary` trait that expresses 
this behavior.

```rust
// src/lib.rs

// A Summary trait that consists of the behavior provided by a summarize method
// We’ve also declared the trait as pub so that crates depending on this crate can make use of this trait too
// A trait can have multiple methods in its body.
pub trait Summary {
    fn summarize(&self) -> String;
}
```

Each type implementing this trait must provide its own custom behavior for the body of the method. 
The compiler will enforce that any type that has the `Summary` trait will have the method summarize defined
with this signature exactly.


Let's implement the Summary trait in our media aggregator:

```rust
// src/libs.rs
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

Now that the library has implemented the `Summary` trait on `NewsArticle` and `Tweet`, users of the crate 
can call the trait methods on instances of `NewsArticle` and `Tweet` in the same way we call regular methods. The only 
difference is that the user must bring the trait into scope as well as the types. 

```rust
use aggregator::{Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}
```

## The orphan rule

One restriction to note is that we can implement a trait on a type only if **either** the trait itself or the type is local to 
our crate. 

For example, we can implement standard library traits like `Display` on a custom type like `Tweet` as part of our 
aggregator crate functionality, because the type `Tweet` is local to our aggregator crate. We can also implement Summary 
on `Vec<T>` in our aggregator crate, because the trait `Summary` is local to our aggregator crate.

This implies that we are unable to implement external traits on external types!

For example, we can’t implement the `Display` trait on `Vec<T>` within our aggregator crate, because `Display`
and `Vec<T>` are both defined in the standard library and aren’t local to our aggregator crate.

This restriction is part of a property called *coherence*, and more specifically *the orphan rule*, so named because 
the parent type is not present. This rule ensures that other people’s code can’t break your code and vice versa. 
**Without the rule, two crates could implement the same trait for the same type, and Rust wouldn't know which 
implementation to use.**

## Default implementations 

Sometimes it’s useful to have **default behavior** for some or all of the methods in a trait instead of requiring 
implementations for all methods on every type. Then, as we implement the `trait` on a particular type, we can keep or 
override each method’s default behavior.

Let's update our `Summary` trait to accomplish this. We now specify a default string for the summarize method of the 
trait instead of only defining the method signature.

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
```

To use a default implementation to summarize instances of `NewsArticle`, we specify an empty `impl `block 
with `impl Summary for NewsArticle {}`.

```rust
// src/libs.rs
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {}

// - snip -
// Creating a default implementation doesn’t require us to change anything about the implementation of Summary on Tweet
```

```rust
fn main() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize()); // New article available! (Read more...)
}
```

## Traits as parameters

We'll use the `Summary` trait we implemented on the `NewsArticle` and `Tweet` types to define a `notify` function 
that calls the `summarize` method on its `item` parameter, which is of some type that **implements** the `Summary` trait. 

```rust
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

Instead of a concrete type for the `item` parameter, we specify the `impl` keyword and the trait name. This parameter 
accepts any type that implements the specified trait. In the body of notify, we can call any methods on item that 
come from the `Summary` trait, such as `summarize`. We can call `notify` and pass in any instance of `NewsArticle` or `Tweet`. 
Code that calls the function with any other type, such as a `String` or an `i32`, won’t compile because those types 
don’t implement `Summary`.

## Trait bound syntax

The `impl Trait` syntax works for straightforward cases but is actually syntax sugar for a longer form known 
as a *trait bound*; it looks like this:

```rust
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

This longer form is equivalent to the example in the previous section but is more verbose. We place trait bounds 
with the declaration of the generic type parameter after a colon and inside angle brackets.

The `impl Trait` syntax is convenient and makes for more concise code in simple cases, while the fuller trait bound 
syntax can express more complexity in other cases. 

```rust
pub fn notify(item1: &impl Summary, item2: &impl Summary)

pub fn notify<T: Summary>(item1: &T, item2: &T) 
```

### Specifying Multiple Trait Bounds with the + Syntax

We can also specify more than one trait bound.

Say we wanted to notify to use display formatting as well as summarize on item: we specify in the notify 
definition that item **must implement** both `Display` and `Summary`. We can do so using the `+` syntax:

```rust
pub fn notify(item: &(impl Summary + Display)) 
```

The `+` syntax is also valid with trait bounds on generic types:

```rust
pub fn notify<T: Summary + Display>(item: &T)
```

With the two trait bounds specified, the body of notify can call `summarize` and use `{}` to format item.

### Clearer Trait Bounds with where Clauses

Using too many trait bounds has its downsides. Each generic has its own trait bounds, so functions with
multiple generic type parameters can contain lots of trait bound information between the function’s name and its parameter 
list, making the function signature hard to read. For this reason, Rust has alternate syntax for specifying trait bounds 
inside a `where` clause after the function signature. So instead of writing this:

```rust
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    // body
}
```

We do this:

```rust
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    /// body
}
```

## Returning Types that Implement Traits

We can also use the `impl Trait` syntax in the return position to return a value of some type that implements a trait:

```rust
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}
```

The `returns_summarizable` function returns some type that implements the `Summary` trait without naming the concrete type.
The ability to specify a return type only by the trait it implements is especially useful in the 
context of **closures and iterators**. Closures and iterators create types that only the compiler knows or types that are 
very long to specify. 

However, you can only use impl Trait if you’re returning a single type. For example, this code that returns either a `NewsArticle` or a `Tweet` 
with the return type specified as impl `Summary` wouldn’t work:

```rust
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }
}
```

This program will not pass the compiler:

```rust
use std::fmt::Display;
fn displayable<T: Display>(t: T) -> impl Display { t }

fn main() {
  let s = String::from("hello");
  let mut s2 = displayable(s);
  s2.push_str(" world");
  println!("{s2}");
}
```

Because displayable returns `impl Display`, then we only know that `s2` is some type that implements `Display`, not 
that it is a `String` which has a `push_str` method. Therefore, we cannot call `s2.push_str(..)`. If the return type 
of displayable was `-> T`, then this program would compile.


## Using Trait Bounds to Conditionally Implement Methods

You can conditionally implement methods for types that satisfy certain trait bounds (or behaviour). 
For instance, we implement `cmp_display` only for types `T` that also implement the `Display` and `PartialOrd` trait.

```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```

We can also conditionally implement a trait for any type that implements another trait. Implementations of a 
trait on any type that satisfies the trait bounds are called **blanket implementations** and are extensively used in 
the Rust standard library. For example, the standard library implements the `ToString` trait on any type that implements 
the `Display` trait. Because the standard library has this blanket implementation, we can call the `to_string` method 
defined by the `ToString` trait on any type that implements the `Display` trait.

```rust
// This trait is automatically implemented for any type which implements the Display trait. As such, 
// ToString shouldn't be implemented directly: Display should be implemented instead, and you get the 
// ToString implementation for free.
impl<T: Display> ToString for T {
    // --snip--
}
```
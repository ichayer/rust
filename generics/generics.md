# Generic data type

We use generics to create definitions for items like function signatures or structs, which we can 
then use with many different concrete data types.

>> If you're finding you need lots of generic types in your code, it could indicate that your
code needs restructuring into smaller pieces.

>> When you recognize situations in your code with multiple struct or enum definitions that differ only in the types of the
values they hold, you can avoid duplication by using generic types instead.

>> Generic types won't make your program run any slower than it would with concrete types.

## In function definitions

When defining a function that uses generics, we place the generics in the signature of the 
function where we would usually specify the data types of the parameters and return value. 
Doing so makes our code more flexible and provides more functionality to callers of our function while 
preventing code duplication.

Let's suppose we need to find the largest `i32` or `char` from a vec:

```rust
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
}
```

The function bodies have the same code, so let’s eliminate the duplication by introducing a 
generic type parameter in a single function.

```rust
// `largest` is generic over some type T.
// This function has one parameter named list, which is a slice of values of type T.
// It will return a reference to a value of the same type T.
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        // ERROR! Function cannot assume anything about the type T! T could be any type: a char, an i32 or even a file!
        // We don't have guarantees that T implements PartialOrd to compare values
        // Rust requires you to state the expected capabilities of generic types up front. So we must restrict T.
        // Check [traits.md](..%2Ftraits.md) to see how to solve it.
        if item > largest { 
            largest = item;
        }
    }

    largest
}
```

Additionally, unlike languages like Java where all objects have a set of core methods like `Object.toString()`, 
there are no core methods in `Rust`. Without restrictions, a generic type `T` has no capabilities: it cannot be printed, 
cloned, or mutated (although it can be dropped).

```rust

fn print_slice<T>(v: &[T]) {
  for x in v {
    println!("{x}"); // ERROR! We cannot assume anything about T, including the ability to turn it into a string.
  }
}
fn main() {
  print_slice(&[1, 2, 3]);
}
```

## In struct definition

We can also define structs to use a generic type parameter in one or more fields.

```rust
// both fields must be that same type
struct Point1<T> {
    x: T,
    y: T,
}

// both fields could have different types
struct Point2<T, U> {
    x: T,
    y: U,
}

fn main() {
    
    // Point 1
    let integer = Point1 { x: 5, y: 10 };
    let float = Point1 { x: 1.0, y: 4.0 };
    // let wont_work = Point { x: 5, y: 4.0 };
    
    // Point 2
    let both_integer = Point2 { x: 5, y: 10 };
    let both_float = Point2 { x: 1.0, y: 4.0 };
    let integer_and_float = Point2 { x: 5, y: 4.0 };
    
}
```

## In Enum Definitions

As we did with structs, we can define enums to hold generic data types in their variants.

We've already seen examples of enums using generics such as `Option<T>` or `Result<T, E>`.

This definition should now make more sense:
- By using the `Option<T>` enum, we can express the abstract concept of an optional value, and because `Option<T>` is generic, 
we can use this abstraction no matter what the type of the optional value is.
- The definition of `Result<T, E>` is convenient because an operation might succeed (return a value of some type `T`)
or fail (return an error of some type `E`)

## In method definitions

We can implement methods on structs and enums and use generic types in their definitions too. 

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
```

We could have chosen a different name for this generic parameter than the generic parameter declared in the
struct definition, but using the same name is conventional. Methods written within an `impl` that declares the generic 
type will be defined on any instance of the type, no matter what concrete type ends up substituting for the generic type.

We can also specify constraints on generic types when defining methods on the type. We could, for example, 
implement methods only on `Point<f32>` instances rather than on `Point<T>` instances with any generic type:

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// An impl block that only applies to a struct with a particular concrete type for the generic type parameter T.
// Other instances of Point<T> where T is not of type f32 will not have this method defined.
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
```

You cannot simultaneously implement specific and generic methods of the same name this way. (i.e. distance_from_origin for
all types T will be rejected by the compiler because Rust does not know which implementation to use!)

```rust
struct Point<T> { x: T, y: T }

impl Point<i32> {
  fn f(&self) -> &i32 { &self.y }
}

impl<T> Point<T> {
  fn f(&self) -> &T { &self.x }
}

fn main() {
  let p: Point<i32> = Point { x: 1, y: 2 };
  println!("{}", p.f());
}
```

These definitions of `f` conflict, and there is no way for Rust to determine which `f` should be used when `p.f()` is called. 
Therefore, this is a compiler error.


## Monomorphization 

As we already mentioned, generic types won't make your program run any slower than it would with concrete types.

Rust accomplishes this by performing monomorphization of the code using generics at compile time. 
Monomorphization is the process of turning generic code into specific code by filling in the concrete types that 
are used when compiled. In this process, the compiler does the opposite of the steps we used to create the generic 
function: the compiler looks at all the places where generic code is called and generates code for the concrete 
types the generic code is called with.

Let’s look at how this works by using the standard library’s generic `Option<T>` enum:

```rust
fn main() {
    let integer = Some(5);
    let float = Some(5.0);
}
```

When Rust compiles this code, it performs monomorphization. During that process, 
the compiler reads the values that have been used in `Option<T>` instances and identifies two kinds of `Option<T>`: 
one is `i32` and the other is `f64`. As such, it expands the generic definition of `Option<T>` into two definitions specialized to `i32` and `f64`, 
thereby replacing the generic definition with the specific ones.

The monomorphized version of the code looks similar to the following (the compiler uses different names than what we’re 
using here for illustration):

```rust
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```

The generic `Option<T>` is replaced with the specific definitions created by the compiler. Because Rust compiles 
generic code into code that specifies the type in each instance, we pay no runtime cost for using generics. 
When the code runs, it performs just as it would if we had duplicated each definition by hand. The process of 
monomorphization makes Rust’s generics extremely efficient at runtime.

BUT:

- Compilation time cost: more code to be produced, more code to be optimized
- Binary size cost: the produced binary will end-up being larger, a typical size/speed trade-off
- Run-time cost: possibly, the larger code size might lead to cache misses at CPU level

# Validating References with Lifetimes

Rather than ensuring that a type has the behavior we want ([traits.md](traits.md)), lifetimes ensure that 
references are valid as long as we need them to be.

Every reference in Rust has a lifetime, which is the scope for which that reference is valid.
Most of the time, lifetimes are implicit and inferred, just like most of the time, types are inferred.
We only must annotate types when multiple types are possible. In a similar way, we must annotate lifetimes when 
the lifetimes of references could be related in a few different ways. Rust requires us to annotate the relationships 
using generic lifetime parameters to ensure the actual references used at runtime will definitely be valid.

## Preventing Dangling References with Lifetimes

The primary purpose of lifetimes in Rust is to avoid situations where references become invalid or 
point to unexpected data (dangling references).

```rust
// An attempt to use a reference whose value has gone out of scope
fn main() {
    let r;
    {
        
        let x = 5;
        r = &x; // borrowed value does not live long enough
        
    } // `x` dropped here while still borrowed
    
    println!("r: {}", r); // - borrow later used here
}
```

`x` doesnt live enough because it will be out of scope when the inner scope ends. 
`r` is still valid for the outer scope because its scope is larger, we say that it “lives longer.” 
If Rust allowed this code to work, `r` would be referencing memory that was deallocated when `x` went out of scope, 
and anything we tried to do with `r` wouldn't work correctly.

## The Borrow Checker Ensures Data Outlives Its References

The Rust compiler's borrow checker will compare scopes to determine whether all borrows are valid.
Let's annotate the lifetime of `r` and `x` with `'a` and `'b` respectively.

```rust
fn main() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}          
```

As we can see, the inner `'b` block is much smaller than the outer `'a` lifetime block. At compile time, 
Rust compares the size of the two lifetimes and sees that `r` has a lifetime of `'a` but that it refers to memory with 
a lifetime of `'b`. The program is rejected because `'b` is shorter than `'a`: the subject of the reference doesn't live as 
long as the reference.

The code below fixes the above code so it doesn't have a dangling reference and compiles without any errors:

```rust
fn main() {
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {}", r); //   |       |
                          // --+       |
}           
```

Here, `x` has the lifetime `'b`, which in this case is larger than `'a`. This means `r` can reference `x` because Rust knows 
that the reference in `r` will always be valid while `x` is valid.

## Generic lifetimes in functions

We’ll write a function that returns the longer of two string slices. This function will take two string slices and 
return a single string slice. 

```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
```

Note that we want the function to take string slices, which are references, rather than strings, 
because we don’t want the longest function to take ownership of its parameters.

However, the implementation of longest won't compile.

The compiler help text reveals that the return type needs a generic lifetime parameter on it because 
Rust can’t tell whether the reference being returned refers to `x` or `y`. Actually, we don’t know either, because 
the `if` block in the body of this function returns a reference to `x` and the `else` block returns a reference to `y`!

When we’re defining this function, we don’t know the concrete values that will be passed into this function, so we don’t 
know whether the `if` case or the `else` case will execute. We also don’t know the concrete lifetimes of the references 
that will be passed in, so we can’t look at the scopes as we did in the examples above to determine whether the reference 
we return will always be valid. The borrow checker can’t determine this either, because it doesn’t know how the lifetimes 
of `x` and `y` relate to the lifetime of the return value. 

To fix this error, we’ll add **generic lifetime parameters** that define the relationship between the references so the 
borrow checker can perform its analysis.

## Lifetime annotation syntax

Lifetime annotations don't change how long any of the references live. Rather, they describe the relationships 
of the lifetimes of multiple references to each other without affecting the lifetimes. Just as functions can 
accept any type when the signature specifies a generic type parameter, functions can accept references with any 
lifetime by specifying a generic lifetime parameter.

Lifetime annotations have a slightly unusual syntax: the names of lifetime parameters must start with an apostrophe (`'`) 
and are usually all lowercase and very short, like generic types. Most people use the name `'a` for the first lifetime 
annotation. We place lifetime parameter annotations after the `&` of a reference, using a space to separate the annotation 
from the reference’s type.

```rust
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```

One lifetime annotation by itself doesn’t have much meaning, because the annotations are meant to tell Rust how generic lifetime 
parameters of multiple references relate to each other.

## Lifetime Annotations in Function Signatures

To use lifetime annotations in function signatures, we need to declare the generic lifetime parameters inside angle brackets 
between the function name and the parameter list, just as we did with generic type parameters.

We want the signature to express the following constraint: the returned reference will be valid as long as both the 
parameters are valid. This is the relationship between lifetimes of the parameters and the return value. We’ll name the 
lifetime `'a` and then add it to each reference.

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

The function signature now tells Rust that for some lifetime `'a`, the function takes two parameters, both of which 
are string slices that live at least as long as lifetime `'a`. The function signature also tells Rust that the string 
slice returned from the function will live at least as long as lifetime `'a`.
In practice, it means that the lifetime of the reference returned by the longest function is the same as the smaller 
of the lifetimes of the values referred to by the function arguments. These relationships are what we want Rust to use 
when analyzing this code.

Remember, when we specify the lifetime parameters in this function signature, we’re not changing the lifetimes of any 
values passed in or returned. We’re specifying that the borrow checker should reject any values that don’t adhere to these 
constraints. Note that the `longest` function doesn't need to know exactly how long `x` and `y` will live, only that some scope 
can be substituted for `'a` that will satisfy this signature.

> > The lifetime annotations become part of the contract of the function, much like the types in the signature.
> > Having function signatures contain the lifetime contract means the analysis the Rust compiler does can be simpler.
> > If there’s a problem with the way a function is annotated or the way it is called, the compiler errors can point to the
> > part of our code and the constraints more precisely.

The lifetime parameter `'a` in the `longest` function is determined based on the actual lifetimes of the references `x` and `y` 
that are passed to it. It will be the part of the scope of `x` that overlaps with the scope of `y`. In other words, 
the generic lifetime `'a` will get the concrete lifetime that is equal to the smaller of the lifetimes of `x` and `y`.
Because we’ve annotated the returned reference with the same lifetime parameter `'a`, the returned reference will be valid 
only for the duration where both `x` and `y` are simultaneously valid. By using the smaller of the two lifetimes, it ensures 
that the reference doesn't outlive either of its input references, thus avoiding dangling references and ensuring memory safety.

```rust
// Using the longest function with references to String values that have different concrete lifetimes
// Borrow checker approves; it will compile and print The longest string is long string is long.
fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}
```

In this example, `string1` is valid until the end of the outer scope, `string2` is valid until the end of the inner scope, 
and `result` references something that is valid until the end of the inner scope.

Next, let’s try an example that shows that the lifetime of the reference in `result` must be the smaller lifetime of the 
two arguments. We’ll move the declaration of the `result` variable outside the inner scope but leave the assignment of the 
value to the `result` variable inside the scope with `string2`. Then we’ll move the `println!` that uses `result` to outside 
the inner scope, after the inner scope has ended. 

```rust
// Attempting to use result after string2 has gone out of scope
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}
```

The error shows that for `result` to be valid for the `println!` statement, `string2` would need to be valid until the end 
of the outer scope. Rust knows this because we annotated the lifetimes of the function parameters and return values using 
the same lifetime parameter `'a`.

As humans, we can look at this code and see that `string1` is longer than `string2` and therefore `result` will contain a 
reference to `string1`. Because `string1` has not gone out of scope yet, a reference to `string1` will still be valid for 
the `println!` statement. However, the compiler can’t see that the reference is valid in this case. We’ve told Rust that 
the lifetime of the reference returned by the `longest` function is the same as the smaller of the lifetimes of the 
references passed in. Therefore, the borrow checker disallows the code above as possibly having an invalid reference.

Let's see another example. This program will not compile:

```rust
fn shortest<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
  if x.len() < y.len() {
    x
  } else {
    y
  }
}

fn main() {
  println!("{}", shortest("hello", "rust"));
}
```

If the type signature says that the function must return a reference with lifetime `'a`, then it would be invalid to 
return a reference with a different lifetime `'b`, i.e. `y` here. In other words, the function is declared to return a 
reference with lifetime `'a`, but it tries to return either `x` or `y`, which may have different lifetimes 
(`'a` and `'b`, respectively).

## Thinking in terms of lifetime

The way in which you need to specify lifetime parameters depends on what your function is doing.
For example, if we changed the implementation of the longest function to always return the first parameter rather 
than the longest string slice, we wouldn't need to specify a lifetime on the y parameter. The following code will compile:

```rust
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
```

We’ve specified a lifetime parameter `'a` for the parameter `x` and the return type, but not for the parameter `y`, because 
the lifetime of `y` does not have any relationship with the lifetime of `x` or the return value.

When returning a reference from a function, the lifetime parameter for the return type needs to match the lifetime parameter 
for one of the parameters. If the reference returned does not refer to one of the parameters, it must refer to a value 
created within this function. However, this would be a dangling reference because the value will go out of scope at the 
end of the function. Consider this attempted implementation of the `longest` function that won’t compile:

```rust
fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str() // returns a reference to data owned by the current function
}
```

Here, even though we’ve specified a lifetime parameter `'a` for the return type, this implementation will fail to compile 
because the return value lifetime is not related to the lifetime of the parameters at all. 

The function `longest` is defined to return a reference with lifetime `'a`. However, the actual data being returned 
(`result.as_str()`) comes from a local variable (`result`) that will be dropped and deallocated when the function exits. 
This creates a problem because the reference being returned would be pointing to memory that has been deallocated, 
leading to a dangling reference.

Additionaly, there's no way to accurately specify the lifetime of the reference (`'a`) to match the actual lifetime of the 
data (result). The lifetime `'a` would need to extend beyond the scope of the function, but Rust's ownership and borrowing 
rules prevent returning references to data that goes out of scope (dangling references).

In this case, we should work with ownership rather than lifetime. This way, the calling function is then responsible for 
cleaning up the value.

>> Lifetime syntax is about connecting the lifetimes of various parameters and return values of functions. Once they’re 
connected, Rust has enough information to allow memory-safe operations and disallow operations that would create dangling 
pointers or otherwise violate memory safety.

## Lifetime annotations in struct definitions

So far, the structs we’ve defined all hold owned types. We can define structs to hold references, but in that case we would 
need to add a lifetime annotation on every reference in the struct’s definition.

```rust
struct ImportantExcerpt<'a> {
    part: &'a str, // struct holds a string slice, which is a reference\
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
```

This annotation means an instance of `ImportantExcerpt` can’t outlive the reference it holds in its `part` field.

## Lifetime Elision

You’ve learned that every reference has a lifetime and that you need to specify lifetime parameters for functions 
or structs that use references.

Lifetime elision in Rust is a set of rules that allow you to omit explicit lifetime annotations in function signatures 
under certain circumstances. These rules were introduced to make code more concise and readable by reducing the need for 
explicit lifetime annotations in references.

The compiler uses three rules to figure out (infer) the lifetimes of the references when there aren’t explicit annotations. 
The first rule applies to input lifetimes (a.k.a parameters), and the second and third rules apply to output lifetimes 
(a.k.a. return value). If the compiler gets to the end of the three rules and there are still references for which 
it can’t figure out lifetimes, the compiler will stop with an error. These rules apply to `fn` 
definitions as well as `impl` blocks.

1. Each parameter with a reference type gets its own lifetime parameter.

   - The function `fn foo(x: &i32)` would get one lifetime parameter and become `fn foo<'a>(x: &'a i32)`. 
   - The function `fn foo(x: &i32, y: &i32)` would get two lifetime parameters and become `fn foo<'a, 'b>(x: &'a i32, y: &'b i32)`. 
   - The function `fn foo(x: &ImportantExcerpt)` would get two lifetime parameters and become 
   `fn foo<'a, 'b>(x: &'a ImportantExcerpt<'b>)`.

2. If there is exactly one input lifetime parameter, it is assigned to all output lifetime parameters.

    - `fn foo<'a>(x: &'a i32) -> &'a i32`.

3. If there are multiple input lifetime parameters, but one of them is `&self` or `&mut self` for a **method**, 
the lifetime of `self` is assigned to all output lifetime parameters.

Let’s pretend we’re the compiler and do some examples. We'll apply these rules to figure out the lifetimes of
the references in some signatures:

```rust
// Compiler receives this function:
fn first_word(s: &str) -> &str 

// Apply first rule:
fn first_word<'a>(s: &'a str) -> &str 

// Apply the second rule (exactly one input lifetime)
fn first_word<'a>(s: &'a str) -> &'a str 
```

Now all the references in this function signature have lifetimes, and the compiler can continue its analysis
without needing the programmer to annotate the lifetimes in this function signature.

Let's look to another example. Let's figure out why `longest` as we code it first does not compile:

```rust
// Compiler receives this function:
fn longest(x: &str, y: &str) -> &str

// Apply first rule:
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str 

// The second rule doesn't apply because there is more than one input lifetime

// The third rule doesn't apply either, because longest is a function rather than a method, so none of the parameters are `self`.
```

After working through all three rules, we still haven’t figured out what the return type’s lifetime is. This is why we got an
error trying to compile the code. The compiler worked through the lifetime elision rules but still couldn't figure out all
the lifetimes of the references in the signature.

```rust
struct Foo<'a> {
  bar: &'a i32
}

fn baz(f: Foo) -> &i32 { /* ... */ }

// Rust will infer this:
fn baz<'a>(f: Foo<'a>) -> &'a i32

// The struct takes a single lifetime parameter, and the output has a single lifetime, 
// so Rust assumes they are the same.
```

```rust
struct Foo<'a> {
  bar: &'a i32
}

// Foo changed to &Foo
fn baz(f: &Foo) -> &i32 { /* ... */ }

// Rust will not compile this program, because it is ambiguous whether the lifetime
// of the output is tied to the lifetime of &Foo or the reference Foo.bar.
```

### Lifetime Annotations in Method Definitions

When we implement methods on a struct with lifetimes, we use the same syntax as that of generic type parameters.
Where we declare and use the lifetime parameters depends on whether they’re related to the struct fields or the method 
parameters and return values.

```rust
// Lifetime names for struct fields always need to be declared after the impl keyword and then used after the struct’s 
// name, because those lifetimes are part of the struct’s type.
impl<'a> ImportantExcerpt<'a> {
   
   // In method signatures inside the impl block, references might be tied to the lifetime of references in the struct’s 
   // fields, or they might be independent.
   
   // We’re not required to annotate the lifetime of the reference to self because it's inferred from the first elision rule.
   // In this case, the return type is i32, which is not a reference, and it is not subject to lifetime elision.
    fn level(&self) -> i32 {
        3
    }
   
}
```

```rust
impl<'a> ImportantExcerpt<'a> {
   
   // There are two input lifetimes, so Rust applies the first lifetime elision rule and gives both &self and announcement 
   // their own lifetimes. Then, because one of the parameters is &self, the return type gets the lifetime of &self, 
   // and all lifetimes have been accounted for.
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
```

## Static lifetime
`'static` lifetime denotes that the affected reference can live for the entire duration of the program. 
All string literals have this lifetime. This implies that the text of the strings are stored directly in the program’s binary.

before specifying 'static as the lifetime for a reference, think about whether the reference you have actually lives the entire 
lifetime of your program or not, and whether you want it to.

Most of the time, an error message suggesting the `'static` lifetime results from attempting to create a dangling reference
or a mismatch of the available lifetimes. In such cases, the solution is fixing those problems, not specifying the `'static`
lifetime.

## Generic Type Parameters, Trait Bounds, and Lifetimes Together

```rust
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
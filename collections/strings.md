# Strings ([string.md](..%2Fstd_library%2Fstring.md) [slice.md](..%2Fownership%2Fslice.md))

Strings are implemented as a collection of bytes. 

New Rustaceans commonly get stuck on strings for a combination of three reasons: Rust’s propensity for 
exposing possible errors, strings being a more complicated data structure than many programmers give them credit for, and UTF-8. 

## What is a string?

Rust has only one string type in the core language, which is the string slice `str` that is usually 
seen in its borrowed form `&str`.

We've already talked about string slices data type (and string literals, but, as they are stored in the program’s binary 
they are string slices too). Slices does not have ownership. Slices let you reference a contiguous sequence of elements in a 
collection rather than the whole collection. Slices are UTF-8 encoded.

A `String` is a growable, mutable, owned, UTF-8 encoded data type.

## Creating a new String

String is actually implemented as a wrapper around a vector of bytes with some extra guarantees, restrictions, and capabilities.
Because of that, we can create a String in the same way as a `Vec<T>`.

This line creates a new empty string called s, which we can then load data into.
```rust
fn main() {
    let mut s = String::new();
}
```

Often, we’ll have some initial data that we want to start the string with. For that, we use the 
`to_string` method, which is available on any type that implements the `Display` trait, as string literals do.

```rust
// Using the to_string method to create a String from a string literal
fn main() {
    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();
}
```

We can also use the function `String::from` to create a `String` from a string literal. The code above is equivalent to the code below:

```rust
fn main() {
    let s = String::from("initial contents");
}
```

## Updating a String

A `String` can grow in size and its contents can change, just like the contents of a `Vec<T>`!

### Concatenation with `push_str` and `push`

We can grow a String by using the `push_str` method to append a string slice.
The method takes a string slice because we don’t necessarily want to take ownership of the parameter

```rust
fn main() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");
}
```

The `push` method takes a single character (`char`) as a parameter and adds it to the `String`. 

```rust
fn main() {
    let mut s = String::from("lo");
    s.push('l');
}
```

## Concatenation with the + operator

Often, you’ll want to combine two existing strings. One way to do so is to use the `+` operator:
```rust
fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
}
```

The reason `s1` is no longer valid after the addition, and the reason we used a reference to `s2`, has to do with the 
signature of the method that’s called when we use the `+` operator. The `+` operator uses the `add` method, whose signature 
looks something like this:

```rust
fn add(self, s: &str) -> String
```

Let's go a bit deeper into this function.

`s2` has an `&`, meaning that we’re adding a reference of the second string to the first string. 
This is because of the `s` parameter in the add function: we can only add a `&str` to a `String`, we can’t add two `String`
values together. But wait, the type of `&s2` is `&String`, not `&str`, as specified in the second parameter.
The reason we’re able to use `&s2` in the call to add is that the compiler can **coerce** the `&String` argument into a `&str`.
When we call the `add` method, Rust uses a **deref coercion**, which here turns `&s2` into `&s2[..]`.
Because add does not take ownership of the `s` parameter, `s2` will still be a valid String after this operation.

Additionally, we can see in the signature that `add` takes ownership of `self`, because `self` does not have an `&`.
This means `s1` will be moved into the add call and will no longer be valid after that producing the following effect:

1. `add` takes ownership of `s1`
2. It appends a copy on the contents of `s2` to `s1`
3. It returns back the ownership of `s1`

If `s1` has enough capacity for `s2`, then no memory allocations occur. However, if `s1` does not have enough capacity for `s2`, 
then `s1` will internally make a larger memory allocation to fit both strings.

## Concatenation with the format! Macro

If we need to concatenate multiple strings, the behavior of the `+` operator gets unwieldy:
```rust
fn main() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
}
```

At this point, `s` will be `tic-tac-toe`. With all of the + and " characters, it’s difficult to see what’s going on.
We can instead use the `format!` macro.

```rust
fn main() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
}
```

This code also sets `s` to `tic-tac-toe`. The `format!` macro works like `println!`, but instead of printing the output 
to the screen, it returns a `String` with the contents. The version of the code using `format!` is much easier to read, 
and the code generated by the `format!` macro uses references so that this call doesn't take ownership of its parameters.

## Indexing into Strings

In many other programming languages, accessing individual characters in a `String` by referencing 
them by index is a valid and common operation. However, if you try to access parts of a `String` using indexing syntax in
Rust, you’ll get an error.

Rust strings don’t support indexing. But why not? Let's discuss how Rust stores Strings in memory.

### Internal representation

As we said, a `String` is a collection of bytes. In Rust, a `String` is a wrapper of `Vec<u8>`.

```rust
fn main() {
    let hello1 = String::from("Hola");
    let hello2 = String::from("Здравствуйте"); // string begins with the capital Cyrillic letter Ze, not the Arabic number 3
}
```

Both "hello1" and "hello2" are examples of properly encoded UTF-8 strings. The fun fact (or maybe not) is analyzing their "length".

In the first case, `len` will be 4, which means the vector storing the `String` "Hola" is 4 bytes long. Each of these letters take
1 byte when encoded in UTF-8.

In the second case, however, you might say that 12 is the length. In fact, Rust’s answer is 24: that’s the number of bytes it 
takes to encode “Здравствуйте” in UTF-8 because each Unicode scalar value in that string takes 2 bytes of storage.
Therefore, an index into the string’s bytes will not always correlate to a valid Unicode scalar value.

### Bytes and Scalar Values and Grapheme Clusters

Another point about UTF-8 is that there are actually three relevant ways to look at strings from Rust’s perspective: 
as bytes, scalar values, and grapheme clusters (the closest thing to what we would call letters).

If we look at the Hindi word “नमस्ते” written in the Devanagari script, it is stored as a vector of u8 values 
that looks like this:

```console
[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
```

That’s 18 bytes and is how computers ultimately store this data. If we look at them as Unicode scalar values, 
which are what Rust’s `char` type is, those bytes look like this:

```console
['न', 'म', 'स', '्', 'त', 'े']
```

There are six `char` values here, but the fourth and sixth are not letters: they’re diacritics that don’t make sense 
on their own. 

Finally, if we look at them as grapheme clusters, we’d get what a person would call the four letters 
that make up the Hindi word:

```console
["न", "म", "स्", "ते"]
```

Rust provides different ways of interpreting the raw string data that computers store so that each program can choose 
the interpretation it needs, no matter what human language the data is in.

A final reason Rust doesn’t allow us to index into a String to get a character is that indexing operations are expected 
to always take constant time (O(1)). But it isn’t possible to guarantee that performance with a String, because Rust would 
have to walk through the contents from the beginning to the index to determine how many valid characters there were.

>> Indexing strings is ambiguous because strings represent several granularities of sequenced data. A UTF-8 string can be 
interpreted as a sequence of bytes, characters, or grapheme clusters. None of these is necessarily the "default" way of 
interpreting a string, so a default indexing operation does not make sense.

## Slicing Strings

Indexing into a string is often a bad idea because it’s not clear what the return type of the string-indexing operation 
should be: a byte value, a character, a grapheme cluster, or a string slice. If you really need to use indices to create 
string slices, therefore, Rust asks you to be more specific.

Rather than indexing using `[]` with a single number, you can use `[]` with a range to create a string slice containing 
particular bytes:

```rust
fn main() {
    let hello = "Здравствуйте";
    let s = &hello[0..4];
}
```

Here, `s` will be a `&str` that contains the first 4 bytes of the string. Earlier, we mentioned that each of these characters 
was 2 bytes, which means s will be `Зд`.

If we were to try to slice only part of a character’s bytes with something like `&hello[0..1]`, 
Rust would panic at runtime in the same way as if an invalid index were accessed in a vector.
You should use ranges to create string slices with caution, because doing so can crash your program.

## Methods for Iterating Over Strings

The best way to operate on pieces of strings is to be explicit about whether you want characters or bytes.

For individual Unicode scalar values, use the `chars` method:
```rust
fn main() {
    // This code prints:
    // З
    // д
    for c in "Зд".chars() {
        println!("{c}");
    }
}
```

Alternatively, the `bytes` method returns each raw byte, which might be appropriate for your domain:

```rust
fn main() {
    // This code prints:
    // 208
    // 151
    // 208
    // 180
    for b in "Зд".bytes() {
        println!("{b}");
    }
}
```
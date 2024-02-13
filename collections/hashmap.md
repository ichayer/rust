# Hash Map

The last of our common collections is the hash map. The type `HashMap<K, V>` stores a mapping 
of keys of type `K` to values of type `V` using a hashing function, which determines how it places these 
keys and values into memory. 

By default, HashMap uses a hashing function called **SipHash**. You can switch to another function by specifying a 
different `hasher` (a type that implements the BuildHasher trait).

Hash maps are useful when you want to look up data not by using an index, as you can with vectors, 
but by using a key that can be of any type. 

Just like vectors, hash maps store their data on the heap, and they are homogeneous: all the keys must have the
same type as each other, and all the values must have the same type.

## Creating a new Hash Map

One way to create an empty hash map is using `new` and adding elements with `insert`.

```rust
use std::collections::HashMap;

// Note that we need to first use the HashMap from the collections portion of the standard library. 
// Of the three common collections, this one is the least often used, so it’s not included in the 
// features brought into scope automatically in the prelude. 

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
}
```

## Accessing Values in a Hash Map

We can get a value out of the hash map by providing its key to the `get` method:

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
}
```

We can iterate over each key/value pair in a hash map in a similar manner as we do with vectors, using a `for` loop:

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores { //  iterating over a hash map happens in an arbitrary order
        println!("{key}: {value}");
    }
}
```

## Ownership

For types that implement the `Copy` trait, like `i32`, the values are copied into the hash map. 
For owned values like `String`, the values will be moved and the hash map will be the owner of those values.

```rust
 use std::collections::HashMap;

fn main() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
}
```

If we insert references to values into the hash map, the values won’t be moved into the hash map. 
The values that the references point to must be valid for at least as long as the hash map is valid.

## Updating a Hash Map

When you want to change the data in a hash map, you have to decide how to handle the case when a key already has a value assigned.

### Overwriting a Value
If we insert a key and a value into a hash map and then insert that same key with a different value, the value 
associated with that key will be replaced.

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores); // This code will print {"Blue": 25}. The original value of 10 has been overwritten.
}
```

## Adding an entry only of the key isn't present

Hash maps have a special API for this called entry that takes the key you want to check as a parameter. 
The return value of the entry method is an enum called `Entry` that represents a value that might or might not exist. 

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50); // insert the key for the Yellow team with the value 50
    scores.entry(String::from("Blue")).or_insert(50); // "Blue" team already has a value. Hash map will not change.

    println!("{:?}", scores); // {"Yellow": 50, "Blue": 10}
}
```

The `or_insert` method on `Entry` is defined to return a mutable reference to the value for the corresponding `Entry`
key if that key exists, and if not, inserts the parameter as the new value for this key and returns a mutable reference 
to the new value.

```rust

use std::collections::HashMap;
fn main() {
  let mut h: HashMap<char, Vec<usize>> = HashMap::new();
    
  for (i, c) in "hello!".chars().enumerate() {
    // "(i, c)" = (index, character)
    // "h.entry(c)": try to get a mutable reference to the value associated with the key c. If the key already exists
    // in the map, it returns the existing entry; 
    // "or_insert": otherwise, it inserts a new entry with the key c associated with an empty Vec<usize>
    // "push(i)": after obtaining or inserting the entry, the index (i) is added to the vector associated with the key c. 
    h.entry(c).or_insert(Vec::new()).push(i);
  }
    
  let mut sum = 0;
  for i in h.get(&'l').unwrap() {
    sum += *i;
  }
    
  println!("{}", sum); // 5
}
```

## Updating a Value Based on the Old Value
Another common use case for hash maps is to look up a key’s value and then update it based on the old value.

```rust
use std::collections::HashMap;

fn main() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map); // {"world": 2, "hello": 1, "wonderful": 1}.
}
```

The `or_insert` method returns a mutable reference (`&mut V`) to the value for the specified key. 
Here we store that mutable reference in the count variable, so in order to assign to that value, 
we must first dereference count using the asterisk (`*`). The mutable reference goes out of scope at the end of 
the `for loop`, so all of these changes are safe and allowed by the borrowing rules.
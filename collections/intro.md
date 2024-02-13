# Collections in Rust

A collection represents a group of objects, known as its elements. 

Unlike the built-in array and tuple types, the data these collections point to is stored on the heap, which means 
the amount of data does not need to be known at compile time and can grow or shrink as the program runs. 

Each kind of collection has different capabilities and costs, and we must choose an appropriate one for the current situation.

## Examples
- A vector allows you to store a variable number of values next to each other.
- A string is a collection of characters.
- A map allows you to associate a value with a particular key.

[std:collections](https://doc.rust-lang.org/std/collections/index.html) specifies other kinds of collections, but they
are "borderline niche collections" used for specific cases. `Vec` and `HasMap` do cover most use cases for generic data storage
and processing.
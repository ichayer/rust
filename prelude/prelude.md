# [Prelude](https://doc.rust-lang.org/std/prelude/index.html)

In Rust, the `prelude` is the set of items that Rust automatically imports into every Rust program. This automatic import is designed to provide a minimal set of elements commonly used in most Rust programs. The prelude is kept as small as possible to avoid unnecessary complexity and focuses on including things, particularly traits, that are utilized in almost every Rust program.

If a type you want to use isn’t in the prelude, you have to bring that type into scope explicitly with a `use` statement. 
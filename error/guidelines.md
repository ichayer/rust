# To `panic!` or not to `panic!`

So how do you decide when you should call `panic!` and when you should return `Result`? 
When code panics, there’s no way to recover. You could call `panic!` for any error situation, whether there’s a 
possible way to recover or not, but then you’re making the decision that a situation is unrecoverable on behalf 
of the calling code. When you choose to return a `Result` value, you give the calling code options. The calling code 
could choose to attempt to recover in a way that’s appropriate for its situation, or it could decide that an `Err` value 
in this case is unrecoverable, so it can call `panic!` and turn your recoverable error into an unrecoverable one. Therefore, 
returning `Result` is a good default choice when you’re defining a function that might fail.

It’s advisable to have your code panic when it’s possible that your code could end up in a bad state. In this context, 
a bad state is when some assumption, guarantee, contract, or invariant has been broken, such as when invalid values, 
contradictory values, or missing values are passed to your code—plus one or more of the following:

- The bad state is something that is unexpected, as opposed to something that will likely happen occasionally, like a user entering data in the wrong format.
- Your code after this point needs to rely on not being in this bad state, rather than checking for the problem at every step.
- There’s not a good way to encode this information in the types you use.

In cases where continuing could be insecure or harmful, the best choice might be to call `panic!` and alert the person using 
your library to the bug in their code so they can fix it during development. Similarly, `panic!` is often appropriate if you’re 
calling external code that is out of your control and it returns an invalid state that you have no way of fixing.
Contracts for a function, especially when a violation will cause a panic, should be explained in the API documentation 
for the function. Take into account that a panic would force the application to only show the panic message, 
and would probably be a worse user experience.

However, when failure is expected, it’s more appropriate to return a `Result` than to make a `panic!` call. Examples include a
parser being given malformed data or an HTTP request returning a status that indicates you have hit a rate limit.
In these cases, returning a `Result` indicates that failure is an expected possibility that the calling code must decide
how to handle and/or perhaps provide additional help like displaying possible solutions or what was expected. 

## Custom types for validation

Creating custom types for validation in Rust allows you to leverage the type system to enforce constraints on data. 
This approach enhances safety, correctness, and expressiveness in your code. By encapsulating validation rules within 
custom types, you promote readability, maintainability, and code reusability across your project. This strategy enables 
early detection of errors related to invalid data, contributing to a more robust and reliable codebase.

```rust
// It’s important that the value field be private so code using the Guess struct is not allowed to set value directly
pub struct CelsiusTemperature {
    value: i32
}

// Implement a validation method for the custom type
// Assuming a valid temperature range between -100 and 100 degrees Celsius
impl CelsiusTemperature {
    pub fn new(value: f32) -> CelsiusTemperature {
        
        // test value, we will panic if temperature violate the contract
        if value >= -100.0 && value <= 100.0 {
            panic!("Celsius temperatura must be between -100 and 100, got {}.", value);
        }
        CelsiusTemperature { value }
    }
    
    // Getter
    pub fn value(&self) -> i32 {
        self.value
    }
}
```


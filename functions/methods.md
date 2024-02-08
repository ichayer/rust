# Associated functions & Methods

Some functions are connected to a particular type. These come in two forms:
associated functions, and methods. Associated functions are functions that
are defined on a type generally (does not receive self as parameter), while methods are associated functions that are
called on a particular instance of a type (receive self as parameter).

>> Methods can only be implemented for types (e.g. impl Point), not variables (like p).

## Method example in a struct

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle { // Everything within this impl block will be associated with the Rectangle type.
    fn area(&self) -> u32 {  // Method receive self as first parameter
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area() // method syntax 
    );

    println!(
        "Can rect1 hold rect2? {}", 
         rect1.can_hold(&rect2)
    );
}
```

Methods can take ownership, borrow immutable or borrow mutably.

If we wanted to change the instance that we’ve called the method on as 
part of what the method does, we’d use `&mut self` as the first parameter. A method takes ownership for example when
the method transforms self into something else, and you want to prevent the caller from using the original instance after the transformation.

## Associated function example in a struct
* Often used for constructors that will return a new instance of the struct.

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let sq = Rectangle::square(3);
}
```

he Self keywords in the return type and in the body of the function are 
aliases for the type that appears after the impl keyword, which in this case is Rectangle.

## Method Calls are Syntactic Sugar for Function Calls

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 { 
        self.width * self.height
    }
}

fn main() {
    let mut r = Rectangle {
        width: 1,
        height: 2
    };
    let area1 = r.area();
    let area2 = Rectangle::area(&r);
    assert_eq!(area1, area2);
}
```

## Methods and Ownership

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    fn max(self, other: Rectangle) -> Rectangle {
        Rectangle {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }
}
```

Methods must be called on structs that have the necessary permissions.

### Read and writes

```rust
fn main() {

    // rect and other_rect have read and ownership permissions
    let rect = Rectangle { width: 0, height: 0 };
    let other_rect = Rectangle { width: 1, height: 1 };

    println!("{}", rect.area()); // Ok! Rect has read permissions
    let max_rect = rect.max(other_rect); // Ok! rect has read and ownership permissions

    rect.set_width(0); // Error. We are missing write permissions (rect is immutable)

    // mutable_rect has read, write and ownership permissions
    let mut mutable_rect = Rectangle { width: 0, height: 0 };

    mutable_rect.set_width(1); // Ok! Rect has write permissions

    let rect_ref = &mutable_rect;

    rect_ref.set_width(2); // Not ok, rect_ref is not a mutable reference.
}
```

Remember: References are non-owning pointers, because they do not own the data they point to.
A reference to a variable can temporarily remove these permissions.

### Moves with self

Calling a method that expects self will move the input struct (unless the struct implements Copy).

```rust
fn main() {
    // rect and other_rect have read and ownership permissions
    let rect = Rectangle { width: 0, height: 0 };
    let other_rect = Rectangle { width: 1, height: 1 };

    let max_rect = rect.max(other_rect); // Ok! rect has read and ownership permissions

    println!("{}", rect.area()); // Error! Rect was moved when calling max.
}
```

Similar situation arises if we try to call a self method on a reference (because reference does not have ownership permission)

#[allow(dead_code)]
#[allow(unused_variables)]

fn main() {
    // Unsafe Rust
    // unsafe Rust doesn't enforce the memory safety guarantees
    // to switch to unsafe Rust, use the `unsafe` keyword and then start a new block that holds the unsafe code
    // you can take five actions in unsafe Rust that you can't in safe Rust:
    // - Dereference a raw pointer
    // - Call an unsafe function or method
    // - Access or modify a mutable static variable
    // - Implement an unsafe trait
    // - Access fields of `union`S
    // it's important to note that `unsafe` doesn't turn off the borrow checker or disable any other of Rust's safety checks

    // examples and discussion: https://rust-book.cs.brown.edu/ch19-01-unsafe-rust.html

    // Advanced Traits
    // scpecifying placeholder types in trait definitions with associated types
    pub trait Iterator {
        type Item;
        fn next(&mut self) -> Option<Self::Item>;
    }

    // operator overloading
    // you can overload the operations and corresponding traits listed in `std::ops`
    use std::ops::Add;
    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }
    impl Add for Point {
        type Output = Point;

        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }
    fn main() {
        assert_eq!(
            Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
            Point { x: 3, y: 3 }
        );
    }

    // default generic type parameters
    // the definition of the `Add` trait uses a defalut generic type `<Rhs = Self>` (Rhs = right hand side) that was used in the above `impl Add for Points` as we wanted to add two `Point` instances
    /* trait Add<Rhs = Self> {
        type Output;
        fn add(self, rhs: Rhs) -> Self::Output;
    } */
    // if we want to use another type for the input then we need to overwrite that default
    struct Millimeters(u32);
    struct Meters(u32);
    impl Add<Meters> for Millimeters {
        type Output = Millimeters;

        fn add(self, other: Meters) -> Millimeters {
            Millimeters(self.0 + (other.0 * 1000))
        }
    }

    // more unusual traits features: https://rust-book.cs.brown.edu/ch19-03-advanced-traits.html

    // Advanced Types
    // type aliases
    type Kilometers = i32;
    // `Kilometers` and `i32` are now the exact same type
    // this is useful to avoid reusing the same long type:
    type Thunk = Box<dyn Fn() + Send + 'static>;
    // `std::io` has this type alias declaration
    type Result<T> = std::result::Result<T, std::io::Error>;

    // the Never type that Never returns
    // the `!` type is an empty type (it doesn't have any value) which in Rust is called the never type
    // expressions of type `!` can be coerced into any other type
    // that's useful in, for example, match expressions, which every branch should return a value of the same type or return a never type, as it will be coerced to the type of the other/s branch/s

    // dynamically sized types and the Sized trait
    // Rust needs to know certain details about its types, such as how much space to allocate for a value of a particular type
    // `str` is a dynamically sized type (DST) or unsized type, which means we can't save values in them
    // `let s: str = "Hello there!";` will not compile as a str can have any length, instead we use slices which have a fixed sized of 2 `usize` (a `&str` stores the starting position and the length)
    let s: &str = "Hello there!";

    // Advanced Functions and Closures
    // function pointers
    // you can pass regular functions to functions, they are coerced to the type `fn`, which is called a function pointer
    fn add_one(x: i32) -> i32 {
        x + 1
    }
    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }
    // unlike closures, `fn` is a type rather than a trait, and it implements all three of the closure traits (`Fn`, `FnMut` and `FnOnce`), meaning you can always pass a function pointer as an argument for a function that expects a closure; it's best to write functions using a generic type and one of the closure traits so your functions can accept either functions or closures

    // returning closures
    // closures are represented by traits, which means you can't return closures directly
    // to work around that we can use a trait object
    fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }

    // Macros
    //The term macro refers to a family of features in Rust: declarative macros with macro_rules! and three kinds of procedural macros:
    //- Custom #[derive] macros that specify code added with the derive attribute used on structs and enums
    //- Attribute-like macros that define custom attributes usable on any item
    //- Function-like macros that look like function calls but operate on the tokens specified as their argument

    // macros are a way of writing code that writes other code (metaprogramming), which is useful for reducing the amount of code you have to write and maintain
    // they seem to have the same purpose of a function, but macros have some additional powers:
    //- a function signature must declare the number and type of parameters the function has; macros on the other hand can take a variable number of parameters (for example, `println!`)
    //- macros are expanded before the compiler interprets the meaning of the code, so a macro can, for example, implement a trait on a given type; funtions can't because they get called at runtime and a trait needs to be implemented at compile time
    //- macros definitions are more complex than function definitions because you are writing Rust code that writes Rust code
    //- macros need to be defined or bring into scope before calling them; functions can be called and defined anywhere

    // declarative macros with `macro_rules!` for generating metaprogramming
    // this is a simplyfied version of the `vec!` macro:
    /* #[macro_export]
    macro_rules! vec {
        ( $( $x:expr ),* ) => { //this is the pattern that matches
            {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push($x);
                )*
                temp_vec
            }
        };
    } */
    // Valid pattern syntax in macro definitions is different than the pattern syntax covered in Chapter 18 because macro patterns are matched against Rust code structure rather than values. For example, declarative macros can match against expressions (expr), types (ty), and even entire items (item).
    // important keys to understand the pattern:
    //- the pattern is always wrapped into ()
    //- the dollar sign is used to declare a variable in the macro system that will contain the Rust code matching the pattern
    //- `$x:expr` matches any Rust expression and gives the expression the name `$x`
    //- the comma following `$()` indicated that a literal comma separator character could optionall appear after the code that matches the code in `$()`
    //- the `*` specifies that the pattern matches zero or more of whatever precedes the `*`; when we call this macro with `vec![1, 2, 3];`, the `$x` pattern matches three times with the three expressions `1`, `2` and `3`
    //- `temp_vec.push()` within `$()*` is generated for each part that matches `$()` in the pattern zero or more times depending on how many times the patters matches, and the `$x` is replaced with each expression matched
    // so finally `vec![1, 2, 3];` generates the code
    /* {
        let mut temp_vec = Vec::new();
        temp_vec.push(1);
        temp_vec.push(2);
        temp_vec.push(3);
        temp_vec
    } */
    // more about macros: https://veykril.github.io/tlborm/

    // procedural macros for generating code from attributes
    // Procedural macros accept some code as an input, operate on that code, and produce some code as an output rather than matching against patterns and replacing the code with other code as declarative macros do. The three kinds of procedural macros are custom derive, attribute-like, and function-like, and all work in a similar fashion.
    // When creating procedural macros, the definitions must reside in their own crate with a special crate type.

    //custom derive macro
    // how to use a derive macro:
    /* use hello_macro::HelloMacro;
    use hello_macro_derive::HelloMacro;

    #[derive(HelloMacro)]
    struct Pancakes;

    fn main() {
        Pancakes::hello_macro();
    } */
    // the derive macro saves the programmer the need to write the following chunk of code for each type that needs it:
    /* impl HelloMacro for Pancakes {
        fn hello_macro() {
            println!("Hello, Macro! My name is Pancakes!");
        }
    } */
    // this is how it's implemented: https://rust-book.cs.brown.edu/ch19-06-macros.html#how-to-write-a-custom-derive-macro
}

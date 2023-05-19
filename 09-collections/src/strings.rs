#![allow(dead_code)]
#![allow(unused_variables)]

pub fn strings() {
    // Rust has onlu one string type in the core language, which is the string slice `str` that is usually seen in its borrowed form &str
    // string slices are references to some UTF-8 encoded string data stored elsewhere
    // for example, string literals are stored in the program's binary and are therefore string slices
    // the `String` type, which is provided by Rust's standar library rather than coded into the core language, is a growable, mutable, owned, UTF-8 encoded string type
    // the `String` type is implemented as a wrapper around a vector of bytes with some extra guarantees, restrictions and capabilities, so it has many of the same operations available with `Vec<T>`

    let mut s = String::new();
}
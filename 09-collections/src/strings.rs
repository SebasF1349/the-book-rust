#![allow(dead_code)]
#![allow(unused_variables)]

pub fn strings() {
    // Rust has onlu one string type in the core language, which is the string slice `str` that is usually seen in its borrowed form &str
    // string slices are references to some UTF-8 encoded string data stored elsewhere
    // for example, string literals are stored in the program's binary and are therefore string slices
    // the `String` type, which is provided by Rust's standar library rather than coded into the core language, is a growable, mutable, owned, UTF-8 encoded string type
    // the `String` type is implemented as a wrapper around a vector of bytes with some extra guarantees, restrictions and capabilities, so it has many of the same operations available with `Vec<T>`

    let s = String::new();

    // to_string() method is available on any type that implements the `Display` trait
    let data = "initial contents";
    let s = data.to_string();

    let mut s = String::from("initial contents");

    s.push_str(": bar"); //for adding strings
    s.push('!'); //for adding chars

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; //s1 has to be moved
    let s1_bis = String::from("Hello, ");
    let s3 = format!("{}{}", s1_bis, s2); //the same as before, but easier for a more complicated string combination - it also uses only references, so this call doesn't take ownersship to any of its parameters

    // Rust doesn't allow for using indexes in strings (`let h = s1[0]` will panic); that's because a string is a Vec<u8> and some chars can take 1 byte long, but others 2 bytes (for example, cyrillic), so taking one byte of a two bytes character wouldn't return the char but "half of it" (or half of its scalar value)

    let hello = "Здравствуйте";
    let s = &hello[0..4]; //returns "Зд", because each character takes 2 bytes

    // you can iterate with `chars()`, which will take the correct amount of loops (2 in this case)
    for c in "Зд".chars() {
        println!("{}", c);
    }
    /* output:
    З
    д */
    // or you can use `bytes()` if you want to use the bytes
    for c in "Зд".bytes() {
        println!("{}", c);
    }
    /* output:
    208
    151
    208
    180 */
}

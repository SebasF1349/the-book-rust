#![allow(dead_code)]
#![allow(unused_variables)]
fn main() {
    // FULL EXPLANATION: https://rust-book.cs.brown.edu/ch04-01-what-is-ownership.html


    // "Rust's goal is to compile programs into efficient binaries that require as few runtime checks as possible. Therefore Rust does not check at runtime whether a variable is defined before being used. Instead, Rust checks at compile-time."

    // "To transfer access to data without copying it, Rust uses pointers. A pointer is a value that describes a location in memory. One common way to make a pointer is to allocate memory in the heap. The heap is a separate region of memory where data can live indefinitely. Heap data is not tied to a specific stack frame. Rust provides a construct called Box for putting data on the heap."

    // "Ownership only exists at compile-time.(...) At runtime, a move is just a copy. At compile-time, a move is a transfer of ownership."

    // "Boxes are used by Rust data structures1 like Vec, String, and HashMap to hold a variable number of elements."

    // using a string in a function moves the ownership of the string to that function scope
    // one way to fix it is cloning (duplicating) the string
    let first = String::from("Ferris");
    let first_clone = first.clone(); //without this line you can't use first in the println! as first would have lost ownership after transferring it to add_sufix
    let full = add_sufix(first_clone);
    println!("{full}, originally {first}");

    // another way is making the fn return the ownership again
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    let (m1_again, m2_again) = greet(m1, m2);
    let s = format!("{} {}", m1_again, m2_again);

    // a less verbose way is using references (i.e. borrowing)
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    greet_two(&m1, &m2);
    let s = format!("{} {}", m1_again, m2_again);


    // FULL EXPLANATION: https://rust-book.cs.brown.edu/ch04-02-references-and-borrowing.html
    // READ IT COMPLETE FOR BETTER UNDERSTANDING, LOTS OF DIAGRAMS TO COPY HERE

    // * is the deference operator
    let mut x: Box<i32> = Box::new(1);
    let a: i32 = *x;         // *x reads the heap value, so a = 1
    *x += 1;                 // *x on the left-side modifies the heap value, so x points to the value 2
    let r1: &Box<i32> = &x;  // r1 points to x on the stack
    let b: i32 = **r1;       // two dereferences get us to the heap value
    let r2: &i32 = &*x;      // r2 points to the heap value directly
    let c: i32 = *r2;        // so only one dereference is needed to read it
    // observation: r1 points to x on the stack, while r2 points directly to the heap

}

fn add_sufix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}

fn greet(g1: String, g2: String) -> (String, String) {
    println!("{} {}!", g1, g2);
    (g1, g2)
}

fn greet_two(g1: &String, g2: &String) {
    println!("{} {}!", g1, g2);
}
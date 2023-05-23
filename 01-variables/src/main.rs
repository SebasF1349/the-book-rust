#![allow(dead_code)]
#![allow(unused_variables)]
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let a = 98_222; //decimal value of all the rest
    let b = 0xff; // hex
    let c = 0o77; // octal
    let d = 0b1111_0000; // binary
    let e = b'A'; // byte (u8 only)

    let f = 2.0; //f64
    let g: f32 = 3.0;

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // results in 0
    let remainder = 43 % 5;

    let t = true; // one byte size

    let c = 'z'; //four bytes size
    let heart_eyed_cat = '😻'; //char represents a Unicode Scalar Value, not just ASCII

    //tuples can group different types, but have a fixed length
    let tup = (500, 6.4, 1);
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; //destructuring
    let five_hundred = tup.0; //five_hundred == x
    let unit = ();

    //arrays items must have the same type and it has a fixed length
    //"Arrays are useful when you want your data allocated on the stack rather than the heap"
    let array = [1, 2, 3, 4, 5];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let array: [i32; 5]; //array without values but with a 5 lenght
    let array = [3; 5]; //[3, 3, 3, 3, 3]
    let first = array[0];
    let second = array[1];
    //accessing an invalid array element cause a panic ON RUNTIME

    //for collections with a variable length you should use vectors instead
}

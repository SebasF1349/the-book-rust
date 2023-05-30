#![allow(dead_code)]
#![allow(unused_variables)]

use std::fmt::Display;

pub fn lifetimes() {
    // lifetimes are another kind of generic that rather than ensuring that a type has the behavious we want, they ensure that references are valid as long as we need them to be

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
    // the lifetime (') is needed because the compiler doesn't know how much time x and y should live, so this notation telss the Rust that x and y should live as long as 'a lives, in oher words, the reference returned by the `longest` fn is the same as the smaller of the lifetimes of the values referred to by the fn args
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    /* "Remember, when we specify the lifetime parameters in this function signature, we’re not changing the lifetimes of any values passed in or returned. Rather, we’re specifying that the borrow checker should reject any values that don’t adhere to these constraints. Note that the longest function doesn’t need to know exactly how long x and y will live, only that some scope can be substituted for 'a that will satisfy this signature." */

    struct ImportantExcerpt<'a> {
        part: &'a str,
    }
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    // lifetimes are sometimes infered by the compiler follwing the next 3 elision rules:
    // 1. the compiler assigns a different lifetime parameter to each lifetime in each input type
    // 2. if there is exactly one input alifetime parameter, that lifetime is assigned to all output lifetime parameters
    // 3. if there are multiple input lifetimes parameters, but one of them is `self` or `&mut self` because this is a method, the lifetime of `self`is assigned to all output lifetime parameters

    impl<'a> ImportantExcerpt<'a> {
        fn level(&self) -> i32 {
            3
        }
        // following rules 1 and 3, all input and output params get the same lifetime
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {}", announcement);
            self.part
        }
    }

    // the `'static` lifetime denotes that the affected reference *can* live for the entire duration of the program
    // all string literals have the `'static`lifetime
    let s: &'static str = "I have a static lifetime";

    // a simple example with generics, trais and lifetimes!
    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: Display,
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}

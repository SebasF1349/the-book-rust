#![allow(dead_code)]
#![allow(unused_variables)]

use std::{
    fs::File,
    io::{Error, ErrorKind, Read},
};

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    // This can be better expressed without match, using unwrap_or_else and closures (see chapter 13), like this:
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // Restult<T,E> has many helpers to make the handling less verbose
    //unwrap returns the Ok variant or it panics when there is any error.
    let greeting_file = File::open("hello.txt").unwrap();
    //expect is the same as unwrap, but let us chose a custom error message
    let greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project");

    // Propagating Errors: make functions returns errors so they can be handled in the call itself, giving more control on the error messages or what to do with the error (for example, create a default username).
    fn read_username_from_file() -> Result<String, Error> {
        let username_file_result = File::open("hello.txt");

        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut username = String::new();

        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }

    //this code can be shortened with the ? operator, this operator at the end of a Result value returns the Ok value or it returns the Err from the whole funtions, as if we have used `returned Err(e)` (using the from function from the From trait in the std lib)
    fn read_username_from_file_shorter() -> Result<String, Error> {
        let mut username_file = File::open("hello.txt")?;
        let mut username = String::new();
        username_file.read_to_string(&mut username)?;
        Ok(username)
    }
    // can even chain methods and make it shorter!
    fn read_username_from_file_even_shorter() -> Result<String, Error> {
        let mut username = String::new();
        File::open("hello.txt")?.read_to_string(&mut username)?;
        Ok(username)
    }

    // in this case, all this operation can be performed in one line with fs::read_to_string instead of File::read_to_string (but then we don't practice error handling):
    fn read_username_from_file_one_liner() -> Result<String, Error> {
        std::fs::read_to_string("hello.txt")
    }

    // ? operator can only be used in functions whose return type is compatible with the value the ? is used on (like Result - that can return an error - or Option - that can return a none -), as it is defined to perform early returns
    // for that reason, `?` can't be used in main()
    // example of `?` with an Option:
    fn last_char_of_first_line(text: &str) -> Option<char> {
        text.lines().next()?.chars().last()
    }

    //to avoid errors, a good idea is to implement validation, this validation can be tedious if it has to be implemented in every function, so a better way to handling it is with custom types.
    // for example, if we need a value from the user to be between 0 and 100 we can create the Guess type as this:
    pub struct Guess {
        value: i32,
    }
    impl Guess {
        pub fn new(value: i32) -> Guess {
            //the same as `value < 1 || value > 100`
            if !(1..=100).contains(&value) {
                panic!("Guess value must be between 1 and 100, for {}", value);
            }
            Guess { value }
        }
        pub fn value(&self) -> i32 {
            self.value
        }
    }
    // the getter is needed because the value is hidden to avoid someone adding a value without using the validation in ::new()
}

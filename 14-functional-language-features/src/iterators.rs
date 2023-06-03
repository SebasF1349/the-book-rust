use std::{env, vec};

#[allow(dead_code)]
#[allow(unused_variables)]

pub fn iterators() {
    // iterators are lazy, they have no effect until you call methods that consume the iterator to use it up
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("Got: {}", val);
    }

    // mut is needed as next changed the internal state that the iterator uses to keep track of where it is in the sequence
    let mut v1_iter = v1.iter();
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    // if we want to create a iterator that takes ownership of v1 and returns owned values, we can call into_iter
    let v1_iter = v1.into_iter();

    // if we want to iterate over mutable references, we can call iter_mut
    let mut v2 = vec![1, 2, 3];
    let v2_iter = v2.iter_mut();

    // iterator methods that call next are called consuming adaptors, because calling them uses up the iterator
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);
    //v1_iter can't be used again as sum took ownership of the iterator we call it on

    // iterators adaptors are methods that don't consume the iterator, instead they produce different iterators by changing some aspect of the original iterator
    let v1 = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    // collect consumes the iterator and returns a vector, in this case with the values of v1 + 1

    struct Shoe {
        size: u32,
        style: String,
    }
    fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        shoes.into_iter().filter(|s| s.size == shoe_size).collect()
    }

    // Let's improve some fn defined in the previous chapter:
    pub struct Config {
        pub query: String,
        pub file_path: String,
        pub ignore_case: bool,
    }
    impl Config {
        // to improve this fn we will change it to take ownership of an iterator as its argument instead of borrowing a slice
        pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
            args.next(); // this is to ignore the first result which is the program name

            let query = match args.next() {
                Some(arg) => arg,
                None => return Err("Didn't get a query string"),
            };
            let file_path = match args.next() {
                Some(arg) => arg,
                None => return Err("Didn't get a file path"),
            };
            // there was no more need of cloning or checking for the number of args!
            let ignore_case = env::var("IGNORE_CASE").is_ok();

            Ok(Config {
                query,
                file_path,
                ignore_case,
            })
        }
    }
    // so now we use the iterator as the argument
    let config = Config::build(env::args()); //and the rest of the unwrap_or_else...

    // we can also use iterators in the search fn to make it clearer
    pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        contents
            .lines()
            .filter(|line| line.contains(query))
            .collect()
    }

    // iterators are zero-cost abstractions, so they are more or less the same as using low-code like for loops (no performance penalty!)
}

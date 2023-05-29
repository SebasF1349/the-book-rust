#![allow(dead_code)]
#![allow(unused_variables)]

use std::fmt::{Debug, Display};

pub fn traits() {
    // a trait defines a functionality a particular type has and can share with other types
    mod trait_example {
        pub trait Summary {
            fn summarize(&self) -> String;
        }
        pub struct NewsArticle {
            pub headline: String,
            pub location: String,
            pub author: String,
            pub content: String,
        }
        impl Summary for NewsArticle {
            fn summarize(&self) -> String {
                format!("{}, by {} ({})", self.headline, self.author, self.location)
            }
        }
        pub struct Tweet {
            pub username: String,
            pub content: String,
            pub reply: bool,
            pub retweet: bool,
        }
        impl Summary for Tweet {
            fn summarize(&self) -> String {
                format!("{}: {}", self.username, self.content)
            }
        }
    }

    use trait_example::{Summary, Tweet}; //the trait must be imported
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize()); //1 new tweet: horse_ebooks: of course, as you probably already know, people

    //traits can only be implemented in a type if the trait or the type is local, and can't be done if both are external (for example, we can't implemente the Display trait in Vec<T> because both are from the std lib)

    // we can also have default behaviours that can later can or not be overwritten
    pub trait SummaryTwo {
        fn summarize(&self) -> String {
            String::from("(Read more...)")
        }
    }

    // and we can have traits with default and not default implementations, that can be called from the default one
    pub trait SummaryThree {
        fn summarize_author(&self) -> String;

        fn summarize(&self) -> String {
            format!("(Read more from {}...)", self.summarize_author())
        }
    }
    // to implement this trait then we only need to define summarize_author

    // we can use traits to define functions that accept many different types
    // this fn accepts any type that implements the Summary trait
    pub fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }
    // that fn signature is syntactic sugar for `pub fn notify<T: Summary>(item &T)`, but there are some differences if we have more than one parameter:
    // this fn accepts two different arguments of different types but with the Summary trait
    pub fn notify_two(item1: &impl Summary, item2: &impl Summary) {}
    // this fn accept two arguments of the same type with the Summary trait
    pub fn notify_three<T: Summary>(item1: &T, item2: &T) {}

    pub fn notify_with_two_traits(item: &(impl Summary + Display)) {}
    pub fn notify_with_two_traits_same_type<T: Summary + Display>(item: &T) {}

    fn fn_many_traits<T, U>(t: &T, u: &U)
    where
        T: Display + Clone,
        U: Clone + Debug,
    {
    }

    fn returns_type_with_trait() -> impl Summary {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }
    // return types with traits are not possible if the fn returns different types (even if all of them have implement the same trait)

    // we can also implement methods only if it has certain traits
    struct Pair<T> {
        x: T,
        y: T,
    }
    impl<T: Display> Pair<T> {}
}

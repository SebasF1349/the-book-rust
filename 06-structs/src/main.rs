#![allow(dead_code)]
#![allow(unused_variables)]

fn main() {
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64
    }

    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someuername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    /* this is the same as:
    let user2 = User {
    active: user1.active,
    username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    }; */
    // we not can't use user1 because the String in the username field of user1 was moved into user2
    // this won't happen if you only have used active and sign_in_count from user1, as those types implement the copy trait

    // tuple structs:
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    // tuple structs are very similar to tuples, but now each one has a different type, which can be useful, for example, for fn arguments

    // unit-like structs
    struct AlwaysEqual;
    let subject = AlwaysEqual;
    // useful when you want to create a triat without any data

    // let's create a program that calculates the area of a rectangle
    let width1 = 30;
    let height1 = 50;
    println!("The area of the rectangle is {} square pixels.", area_variables(width1, height1));
    fn area_variables(width: u32, height: u32) -> u32 {
        width * height
    }
    // the issue with this area fn is that the width and height are not apparently related, using structs give more meaning to the variables used
    struct Rectangle {
        width: u32,
        height: u32,
    }
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is {} square pixels.", area_struct(&rect1));
    fn area_struct (rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }
    // as the area fn is closely related to the struct, makes sense to add it as a method
    struct RectangleTwo {
        width: u32,
        height: u32,
    }
    impl RectangleTwo {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }
    let rect2 = RectangleTwo {
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is {} square pixels.", rect2.area());
    // let's create a new method to find out if a rectangle fits into another one
    impl RectangleTwo {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }
    // if the implemented fn doesn't have self as their first parameter, then it's not a method but an associated function, for example a constructor:
    impl RectangleTwo {
        fn square(size: u32) -> Self {
            Self { width: size, height: size }
        }
    }
    let sq = RectangleTwo::square(3); //associated fn are called with :: (as String::from() which is an associated fn!)
    // in fact, doing `rect2.area()` as before is syntactic Sugar for `RectangleTwo::area(&rect2)`

    // to print a struct we need to manually add the Debug attribute
    #[derive(Debug)]
    struct RectanglePrint {
        width: u32,
        height: u32,
    }
    let rect2 = RectanglePrint {
        width: 30,
        height: 50,
    };
    println!("rect1 is {:?}", rect2); //using {:#?} will make an even prettier print for langer structs

}

#![allow(dead_code)]
#![allow(unused_variables)]

fn main() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    impl Message {
        fn call(&self) {
            // method body
        }
    }
    let m = Message::Write(String::from("hello"));
    m.call();

    // Options are a special type of enum and encodes the very common scenario in which a value could be something or it could be nothing (null does not exist in Rust)
    /* enum Option<T> {
        None,
        Some(T),
    } */
    // Option doesn't need to be bringed into scope explicitly, `Some` and `None` can be used directly (instead of Option::Some or Option::None)
    // the Option Type can be infered only if you give the value of some
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None; //type must be explicit if we only define None
                                           // Option help catching one of the most common issues with null: assuming that something isn't null when it actually is

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }
    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }

    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
        // all US states
    }
    enum CoinForStates {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }
    fn value_in_cents_states(coin: CoinForStates) -> u8 {
        match coin {
            CoinForStates::Penny => 1,
            CoinForStates::Nickel => 5,
            CoinForStates::Dime => 10,
            CoinForStates::Quarter(state) => {
                println!("State quarter from {:?}", state);
                25
            }
        }
    }

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
    let five = Some(5);
    let fix = plus_one(five);
    let none = plus_one(None);

    // match must cover all possibilities or will get a compile error
    // how to catch all other posibilities in a match:
    // other => function(other),
    // _ => function(),
    // _ => (),

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
    // `if let` is syntactic sugar for a match that runs code when the value matchs one pattern and then ignores all other values; is this case, the `if let` is the same as:
    /* match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    } */
    // the `_ => ...` can be added to the else part of the if let
}

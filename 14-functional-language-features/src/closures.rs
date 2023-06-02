use std::{thread, time::Duration};

#[allow(dead_code)]
#[allow(unused_variables)]

pub fn closures() {
    #[derive(Debug, PartialEq, Clone, Copy)]
    enum ShirtColor {
        Red,
        Blue,
    }

    struct Inventory {
        shirts: Vec<ShirtColor>,
    }

    impl Inventory {
        fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
            user_preference.unwrap_or_else(|| self.most_stocked())
            // `|| self.most_stocked()` is a closure! It takes no parameters (they would be between the |), the body calls self.most_stocked() and the implementation of unwrap_or_else will evaluate the closure later if the result is needed
        }

        fn most_stocked(&self) -> ShirtColor {
            let mut num_red = 0;
            let mut num_blue = 0;

            for color in &self.shirts {
                match color {
                    ShirtColor::Red => num_red += 1,
                    ShirtColor::Blue => num_blue += 1,
                }
            }
            if num_red > num_blue {
                ShirtColor::Red
            } else {
                ShirtColor::Blue
            }
        }
    }

    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );
    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly");
        thread::sleep(Duration::from_secs(2));
        num
    };

    // this fn and closure do the same
    fn add_one_v1(x: u32) -> u32 {
        x + 1
    }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };

    // closure types can be infered (not possible with fn)
    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));

    // the closure can capture values from their environment in three ways: borrowing immutably, borrowing mutably and taking ownership; the closure will decide which of these to use based on what the body of the function does with the captured values
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    let only_borrows = || println!("From closure: {:?}", list);
    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
    // if only_borrows would mutate the list (with a push for example), then it will make a mut borrowing, making impossible to print between the definition and the calling of it. After the callinf printing would be allowed again as the mut bottow would end.

    // the move keyword to force the closure to take ownership of the values it uses in the environment
    // this is needed, for example, when creating a new thread (to avoid panic if the main thread finishes before the spawned thread)
    println!("Before defining closure: {:?}", list);
    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();

    // Closures will automatically implement one, two or all three of these `Fn` traits, in an additiva fashion
    // 1.`FnOnce` applies to closures that can be called at least once. All closures implement at least this trait
    // 2.`FnMut` applies to closures that don't move captures values out of their body, but that might mutate the captures values and so can be called more than once.
    // 3.`Fn`applies to closures that don't move captured values out of their body and that don't mutate captures values, as well as closures that capture nothing from their environment.
    // note: fn can implement all three of the `Fn` traits too
}

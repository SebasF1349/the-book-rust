#[allow(dead_code)]
#[allow(unused_variables)]

fn main() {
    // `match` expressions need to be exhaustive
    /* match x {
        None => None,
        Some(i) => Some(i + 1),
    } */

    // `if let` expressions are used mainly as a shorter way to write the equivalent of a match that only matches one case. Optionally, if let can have a corresponding else containing code to run if the pattern in the if let doesn’t match.
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();
    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    // a `while let` conditional loop allows a `while` loop to run for as long as a pattern continues to match
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // in a `for` loop, the value that directly follows the keyword `for` is a pattern
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // `let` statements also use patterns in the form `let PATTERN = EXPRESSION;`
    let x = 5;
    let (x, y, z) = (1, 2, 3);

    // fn params can also be patterns
    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({}, {})", x, y);
    }

    // patterns come in two forms: refutable and irrefutable. Patterns that will match for any possible value passed are irrefutable. An example would be x in the statement let x = 5; because x matches anything and therefore cannot fail to match. Patterns that can fail to match for some possible value are refutable
    // fn params, let statements and for loops can only accept irrefutable patterns, because the program cannot do anything meaningful when values don't match
    // the if let and while let expressions accept refutable and irrefutable patterns, but the compiler warns against irrefutable patterns becaise by definition they are intended to handle possible failure

    // Pattern Syntax

    // Matching Literals
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // Matching Named Variables
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"), // this will be printed, as this is a new y that shadows the `y = 10`
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {y}", x);

    // Multiple Patterns
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // Matching Ranges
    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
    // can also be used with chars!
    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // Destructuring
    // destructuring structs
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
    //if the variable name is equal to the key name, we can use the shorthand `let Point {x,y} = p`
    // we can also destructure with literal values
    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }
    // destructuring enums
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g}, and blue {b}",)
        }
    }
    // destructurng nested structs and enums
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }
    enum MessageNested {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }
    let msg = MessageNested::ChangeColor(Color::Hsv(0, 160, 255));
    match msg {
        MessageNested::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        MessageNested::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}")
        }
        _ => (),
    }
    // destructuring structs and tuples
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    // Ignoring Values
    fn foo(_: i32, y: i32) {
        println!("This code only uses the y parameter: {}", y);
    }
    // ignoring parts of a value with a nested _
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting is {:?}", setting_value);
    // ignoring parts of a value with ..
    struct PointThree {
        x: i32,
        y: i32,
        z: i32,
    }
    let origin = PointThree { x: 0, y: 0, z: 0 };
    let PointThree { x, .. } = origin;
    println!("x is {}", x);

    let numbers = (2, 4, 8, 16, 32);
    let (first, .., last) = numbers;
    println!("Some numbers: {first}, {last}");

    // Extra Conditionals with Match Guards
    // a match guard is an additional if condition, specified after the pattern in a match arm, that must also match for that arm to be chosen
    // match guards are useful for expressing more complex ideas than a pattern alone allows
    let num = Some(4);
    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }
    // match guards allow us to compare with outer variables
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {y}", x); // Default case, x = Some(5)

    // @ Bindings
    // the at operator @ lets us create a variable that holds a value at the same time as we’re testing that value for a pattern match
    enum MessageHello {
        Hello { id: i32 },
    }
    let msg = MessageHello::Hello { id: 5 };
    match msg {
        MessageHello::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        MessageHello::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        MessageHello::Hello { id } => println!("Found some other id: {}", id),
    } // Found an id in range: 5
}

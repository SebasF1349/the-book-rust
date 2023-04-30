//STATEMENTS are instructions that perform some action and do not return a value
//EXPRESSIONS evaluate to a resulting value
//EXPRESSIONS do not include ending semicolons, adding a semicolon turns it into a statement (no there is no resulting value!)

fn main() {
    println!("Hello, world!");

    let x = another_function(5, 'h'); // x == 5
}

fn another_function(value: i32, unit_label: char) -> i32 {
    println!("The measurement is: {value}{unit_label}");
    value
    //returns the value - it's an implicit return as it doesn't use the return keyword nor semicolon - `return value;` would be the same but explicitly
    //adding a semicolon will show a "mismatched types" error while compiling, as it would return a unit `()` while the fn return type should be i32
}
fn main() {
    let number = 3;
    //`if number {}` will give an error, as number is not boolean
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }


    //`if` is an expresion, so can be used in the right side of a let
    let condition = true;
    let number = if condition { 5 } else { 6 };

    /* loop {
        println!("again!")
    } */


    //use a value after a break to return it
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2
        }
    };
    println!("The result is {result}");


    //loops can be labeled to break or continue specific loops
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up //AMAZING
            }
            remaining = -1;
        }
        count += 1;
    }
    println!("End count = {count}");


    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("LIFTOFF!!!");


    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!!");


    let a = [10, 20, 30, 40 , 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }


    let a = [10, 20, 30, 40 , 50];
    for element in a {
        println!("the value is {element}");
    }
}

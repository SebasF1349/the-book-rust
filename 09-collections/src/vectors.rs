#![allow(dead_code)]
#![allow(unused_variables)]

pub fn vector() {
    let v1: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];
    let mut v3 = Vec::new(); //infers the type because of the next line
    v3.push(5);

    let third = &v2[2]; //this doesn't care if the index is valid
    let second = v2.get(2); // this returns an option

    // you can't mutate a vector that has been borrowed, even if you mut another index, because changing sizes can lead to reallocating while the previous reference won't work anymore

    for n_ref in &v2 {
        let plus_one = *n_ref + 1;
        println!("{}", plus_one);
    }

    for n_ref in &mut v3 {
        *n_ref += 50;
    }

    // we can use enums to save values of different types in a vector
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    // this works because all the data is of the same type: SpreadsheetCell (!)
}

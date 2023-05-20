#![allow(dead_code)]
#![allow(unused_variables)]


pub fn hash_maps() {
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Hashmaps are saved on the heap and are homogeneous: all the keys must have the same type as each other, and the same goes for their values

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (value, key) in &scores {
        println!("{}: {}", key, value);
    }

    // updating a hash map:
        // saving a value and overwriting if there is an old value
        scores.insert(String::from("Blue"), 25);
        // only save a new value if the key doesn't exists
        scores.entry(String::from("Blue")).or_insert(25);
        // updating a value based on the old value
        // example: counting repeated words in a string
        let text = "hello world wonderful world";
        let mut map = HashMap::new();
        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }
}
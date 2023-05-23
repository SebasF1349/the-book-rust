#![allow(dead_code)]
#![allow(unused_variables)]

mod hash_maps;
mod strings;
mod vectors;

use std::{
    collections::{hash_map, HashMap},
    io,
};

fn main() {
    // teoria en los modulos
    // Ejercicios:
    // 1. Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list.
    let mut nums = vec![4, 3, 2, 5, 4, 4, 2, 2, 7, 5, 4];
    nums.sort();
    let length = if nums.len() % 2 == 0 {
        nums.len() / 2
    } else {
        (nums.len() + 1) / 2
    };
    let median = nums[length];
    let mut counter = HashMap::new();
    let mut max_quantity = 0;
    let mut max_number = 0;
    for num in nums {
        let n = counter.entry(num).and_modify(|num| *num += 1).or_insert(1);
        if n > &mut max_quantity {
            max_quantity = *n;
            max_number = num;
        }
    }
    println!(
        "La mediana de los número es {} y la moda es {}",
        median, max_number
    );

    // 2. Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
    let text = String::from("first");
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];
    let pig_latin = if !vowels.contains(&text.chars().next().unwrap()) {
        format!("{}-{}ay", &text[1..], text.chars().next().unwrap())
    } else {
        format!("{}-hay", &text)
    };
    println!("{}", pig_latin);

    // 3. Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
    let mut data: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        println!("Ingrese usuario (o escriba `n` para dejar de ingresar): ");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();
                if input == "n" {
                    break;
                }
                match &input.find("to") {
                    Some(pos) => {
                        let name = String::from(&input[4..pos - 1]);
                        let departament = String::from(&input[pos + 3..]);
                        if let hash_map::Entry::Vacant(e) = data.entry(departament.clone()) {
                            e.insert(vec![name]);
                        } else {
                            data.insert(departament, vec![name]);
                        }
                    }
                    _ => {
                        println!("Formato incorrecto.");
                    }
                }
            }
            Err(error) => println!("Error: {}", error),
        };
    }
    println!("Ingrese el nombre de un departamento para ver sus empleados o deje en blanco para verlos a todos: ");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let input_trimmed = input.trim();
            if data.contains_key(input_trimmed) {
                for name in data.get(input_trimmed).unwrap() {
                    println!("{}", name);
                }
            } else {
                for (k, v) in &data {
                    println!("Department: {}", k);
                    for name in v {
                        println!("{}", name);
                    }
                }
            }
        }
        Err(error) => println!("Error: {}", error),
    }
}

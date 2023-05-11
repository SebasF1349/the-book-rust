#![allow(dead_code)]
#![allow(unused_variables)]

fn main() {
    // Ranges
    // [starting_index..ending_index], starting_index is included in the range, but not ending_index
    // [..2] is equal to [0..2] and [3..] equals to [3..(length)] (and so [..] means the entire string)

    // Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection. A slice is a kind of reference, so it is a non-owning pointer
    let s = String::from("hello world");
    let hello: &str = &s[0..5]; //a string slice (a reference to part of the `s`); as it's a reference not `s` lost write and own permissions
    let world = &s[6..11];
    let s2: &String = &s; //a reference to the whole `s`
    // slices are "fat" pointers or pointers with metadata (the length of the slice)

    fn first_word(s: &String) -> &str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        &s[..]
    }

    // String Literals *are* string slices and string references are equivalent to whole slices, so the previous fn signatura can be rewritten to
    fn fist_word_two(s: &str) -> &str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        &s[..]
    }
    // and now this fn can take any "type of string"

    let my_string = String::from("hello world");
    // `first_word` works on slices of `String`s, whether partial or whole
    let word = fist_word_two(&my_string[0..6]);
    let word = fist_word_two(&my_string[..]);
    // `fist_word_two` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = fist_word_two(&my_string);

    let my_string_literal = "hello world";
    // `fist_word_two` works on slices of string literals, whether partial or whole
    let word = fist_word_two(&my_string_literal[0..6]);
    let word = fist_word_two(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = fist_word_two(my_string_literal);

    // apart from string slices, you can also refer to a part of an array (or other collections):
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);

    // NOTE: as slices also save the length, then a whole slice is bigger (in bytes) than a reference.
}

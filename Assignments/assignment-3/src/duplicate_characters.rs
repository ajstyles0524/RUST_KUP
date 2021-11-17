use log::*;
// find all duplicates characters from a given string
// &str = stack
// String = heap
pub fn duplicate_char() {
    let string = String::from("Bingo Boy");
    //indexing into a string is not available in Rust
    // string supports iteration but not indexing, therefore i convert string into list of chars
    // therefore for indexing purpose, i convert given string into list of char
    let vec_string: Vec<char> = string.chars().collect();
    for index_1 in 0..string.len() {
        for index_2 in index_1 + 1..string.len() {
            if vec_string[index_1] == vec_string[index_2] {
                debug!("{}", vec_string[index_2]);
            }
        }
    }
}

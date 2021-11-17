use log::*;
mod duplicate_characters;
mod is_palindrome;
mod rotate_string;
fn main() {
    env_logger::init();
    duplicate_characters::duplicate_char();
    debug!("{}", rotate_string::rotate_check("abcd", "dcba"));
    let string = "ANAND";
    if is_palindrome::is_palindrome(string) {
        debug!("Palindrome");
    } else {
        debug!("Not Palindrome");
    }
}

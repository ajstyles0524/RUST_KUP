use log::*;
pub fn rotate_check(string1: &str, string2: &str) -> bool {
    let mut collect_reverse_char = String::new();
    for count in string1.chars().rev() {
        collect_reverse_char.push(count);
    }
    info!("Rotation of two string is performed");
    collect_reverse_char == string2

}

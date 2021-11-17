pub fn is_palindrome(string: &str) -> bool {
    let vec_string: Vec<char> = string.chars().collect();
    let mut low = 0;
    let mut high = string.len() - 1;
    while low < high {
        if vec_string[low as usize] != vec_string[high as usize] {
            return false;
        }
        low += 1;
        high -= 1;
    }
    true
}

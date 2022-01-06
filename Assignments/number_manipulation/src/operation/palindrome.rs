use log::*;
/// check_palindrome function is used to check a given number is palindrome number or not
///
/// #Arguments
///
/// number: number is a i32 integer type input number
///
/// #Return
///
/// Return bool type
pub fn check_palindrome(number: i32) -> bool {
    let mut temp: i32 = number;
    let mut remainder: i32;
    let mut reversed: i32 = 0;
    info!("Calculating reverse of the given number");
    while temp != 0 {
        remainder = temp % 10;
        reversed = reversed * 10 + remainder;
        temp /= 10;
    }
    info!("status of the given number is palindrome number or not is returned");
    reversed == number
}

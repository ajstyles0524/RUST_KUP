use log::*;
/// check_armstrong function is used to check a given number is armstrong number or not
///
/// #Arguments
///
/// number: number is a i32 integer type input number
///
/// #Return
///
/// Return bool type
pub fn check_armstrong(number: i32) -> bool {
    let mut count = 0;
    let mut temp_1 = number;
    let mut temp_2 = number;
    info!("Counting the number of digit in the given number");
    while temp_1 != 0 {
        temp_1 /= 10;
        count += 1;
    }
    let mut result: i32 = 0;
    let mut remainder: i32;
    while temp_2 != 0 {
        remainder = temp_2 % 10;
        result += power(remainder, count);
        temp_2 /= 10;
    }
    info!("status of the given number is armstrong or not is returned");
    result == number
}
/// power function is used to calculate power of the number for the given number
///
/// #Arguments
///
/// number_1: number_1 is a i32 integer type input number
/// number_2: number_2 is a i32 integer type input number
///
/// #Return
///
/// Return i32 type
pub fn power(number_1: i32, number_2: i32) -> i32 {
    let mut result = 1;
    for _index in 0..number_2 {
        result *= number_1;
    }
    result
}

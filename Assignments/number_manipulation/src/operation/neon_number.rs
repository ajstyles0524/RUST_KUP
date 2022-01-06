use log::*;
/// check_neon function is used to check a given number is neon number or not
///
/// #Arguments
///
/// number: number is a i32 integer type input number
///
/// #Return
///
/// Return bool type
pub fn check_neon(number: i32) -> bool {
    let mut square = number * number;
    let mut sum: i32 = 0;
    while square > 0 {
        sum += square % 10;
        square /= 10;
    }
    info!("status of the given number is neon number or not is returned");
    number == sum
}

use log::*;
/// check_automorphic function is used to check a given number is automorphic number or not
///
/// #Arguments
///
/// number: number is a i32 integer type input number
///
/// #Return
///
/// Return bool type
pub fn check_automorphic(mut number: i32) -> bool {
    let mut square = number * number;
    while number > 0 {
        if number % 10 != square % 10 {
            return false;
        }
        number /= 10;
        square /= 10;
    }
    info!("status of the given number is automorphic or not is returned");
    true
}

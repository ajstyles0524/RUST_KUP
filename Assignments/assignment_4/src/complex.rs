use log::*;
/// Complex number structure
pub struct Complex {
    pub real: f64,
    pub img: f64,
}
/// addition function add two  complex numbers using structures to function
///
/// #Arguments
///
/// Reference of two complex number
///
/// #Return
///
/// No return
pub fn addition(complex_1: &Complex, complex_2: &Complex) {
    if complex_1.img + complex_2.img >= 0.0 {
        debug!(
            "Sum of the complex numbers = {} + {}i",
            complex_1.real + complex_2.real,
            complex_1.img + complex_2.img
        );
    } else {
        debug!(
            "Sum of the complex numbers = {} {}i",
            complex_1.real + complex_2.real,
            complex_1.img + complex_2.img
        );
    }
}
/// subtraction function subtract two complex numbers using structures to function
///
/// #Arguments
///
/// Reference of two complex number
///
/// #Return
///
/// No return
pub fn subtraction(complex_1: &Complex, complex_2: &Complex) {
    if complex_1.img + complex_2.img >= 0.0 {
        debug!(
            "Difference of the complex numbers = {} + {}i",
            complex_1.real - complex_2.real,
            complex_1.img - complex_2.img
        );
    } else {
        debug!(
            "Difference of the complex numbers = {} {}i",
            complex_1.real - complex_2.real,
            complex_1.img - complex_2.img
        );
    }
}
/// multiplication function multiply two complex numbers using structures to function
///
/// #Arguments
///
/// Reference of two complex number
///
/// #Return
///
/// No return
pub fn multiplication(complex_1: &Complex, complex_2: &Complex) {
    if complex_1.img + complex_2.img >= 0.0 {
        debug!(
            "Multiplication of the complex numbers = {} + {}i",
            complex_1.real * complex_2.real,
            complex_1.img * complex_2.img
        );
    } else {
        debug!(
            "Multiplication of the complex numbers = {} {}i",
            complex_1.real*complex_2.real - complex_1.img*complex_2.img , complex_1.real*complex_2.img + complex_1.img*complex_2.real
        );
    }
}
pub mod test;
pub mod operation {
    pub mod armstrong;
    pub mod automorphic;
    pub mod matrix_multiplication;
    pub mod neon_number;
    pub mod palindrome;
}
use log::*;
use text_io::read;
use crate::operation::armstrong::check_armstrong;
use crate::operation::automorphic::check_automorphic;
use crate::operation::matrix_multiplication::Matrix;
use crate::operation::neon_number::check_neon;
use crate::operation::palindrome::check_palindrome;

fn main() {
    env_logger::init();
    loop {
        info!(
            "------Mathematics Menu------ \n
    1:- Check whether the given number is Armstrong number or not\n
    2:- Check whether the given number is Automorphic number or not\n
    3:- Check whether the given number is Neon number or not\n
    4:- Check whether the given number is Palindrome number or not\n
    5:- Perform matrix multiplication\n
    (Press -1 to exit)\n"
        );
        let choice: i32 = read!();
        if choice == 1 {
            info!("Enter a number",);
            let number: i32 = read!();
            debug!("{}", check_armstrong(number));
        }
        if choice == 2 {
            info!("Enter a number");
            let number: i32 = read!();
            debug!("{}", check_automorphic(number));
        }
        if choice == 3 {
            info!("Enter a number");
            let number: i32 = read!();
            debug!("{}", check_neon(number));
        }
        if choice == 4 {
            info!("Enter a number");
            let number: i32 = read!();
            debug!("{}", check_palindrome(number));
        }
        if choice == 5 {
            let first = Matrix {
                element: [[1, 2, 3], [4, 5, 6], [7, 8, 9]],
            };
            let second = Matrix {
                element: [[1, 0, 0], [0, 1, 0], [0, 0, 1]],
            };
            let result = Matrix::matrix_multiplication(first, second);
            debug!("{:?}",result);
        }
        if choice == -1 {
            break;
        }
    }
}

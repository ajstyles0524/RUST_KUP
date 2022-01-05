#[cfg(test)]
pub mod tests {
    use crate::operation::{
        armstrong, automorphic, matrix_multiplication::Matrix, neon_number, palindrome,
    };
    #[test]
    fn armstrong_success() {
        assert_eq!(armstrong::check_armstrong(1634), true);
    }
    #[test]
    fn armstrong_failure() {
        assert_eq!(armstrong::check_armstrong(1500), false);
    }
    #[test]
    fn automorphic_success() {
        assert_eq!(automorphic::check_automorphic(5), true);
    }
    #[test]
    fn automorphic_failure() {
        assert_eq!(automorphic::check_automorphic(7), false);
    }
    #[test]
    fn neon_success() {
        assert_eq!(neon_number::check_neon(9), true);
    }
    #[test]
    fn neon_failure() {
        assert_eq!(neon_number::check_neon(12), false);
    }
    #[test]
    fn palindrome_success() {
        assert_eq!(palindrome::check_palindrome(1221), true);
    }
    #[test]
    fn palindrome_failure() {
        assert_eq!(palindrome::check_palindrome(1222), false);
    }
    #[test]
    fn multiplication_first() {
        let first = Matrix {
            element: [[1, 2, 3], [4, 5, 6], [7, 8, 9]],
        };
        let second = Matrix {
            element: [[1, 0, 0], [0, 1, 0], [0, 0, 1]],
        };
        let left = Matrix::matrix_multiplication(first, second);
        let right = Matrix {
            element: [[1, 2, 3], [4, 5, 6], [7, 8, 9]],
        };
        assert_eq!(left, right);
    }
    #[test]
    fn multiplication_second() {
        let first = Matrix {
            element: [[1, 1, 1], [1, 1, 1], [1, 1, 1]],
        };
        let second = Matrix {
            element: [[1, 1, 1], [1, 1, 1], [1, 1, 1]],
        };
        let left = Matrix::matrix_multiplication(first, second);
        let right = Matrix {
            element: [[3, 3, 3], [3, 3, 3], [3, 3, 3]],
        };
        assert_eq!(left, right);
    }
}

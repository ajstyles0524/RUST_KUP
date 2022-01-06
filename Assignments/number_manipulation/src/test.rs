#[cfg(test)]
pub mod tests {
    use crate::operation::{
        armstrong, automorphic, matrix_multiplication::matrix_multiplication, neon_number,
        palindrome,
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
    fn multiplication_first_success() {
        let first = [[1, 2, 3].to_vec(), [4, 5, 6].to_vec(), [7, 8, 9].to_vec()].to_vec();
        let second = [[1, 0, 0].to_vec(), [0, 1, 0].to_vec(), [0, 0, 1].to_vec()].to_vec();
        let right = [[1, 2, 3].to_vec(), [4, 5, 6].to_vec(), [7, 8, 9].to_vec()].to_vec();
        assert_eq!(matrix_multiplication(&first, &second), Ok(right));
    }
    #[test]
    fn multiplication_second_success() {
        let first = [[1, 2].to_vec(), [3, 4].to_vec()].to_vec();
        let second = [[1, 0].to_vec(), [0, 1].to_vec()].to_vec();
        let right = [[1, 2].to_vec(), [3, 4].to_vec()].to_vec();
        assert_eq!(matrix_multiplication(&first, &second), Ok(right));
    }
    #[test]
    fn multiplication_first_failure() {
        let first = [[1, 2].to_vec(), [3, 4].to_vec()].to_vec();
        let second = [[1, 2, 3].to_vec(), [4, 5, 6].to_vec(), [7, 8, 9].to_vec()].to_vec();
        assert_eq!(
            matrix_multiplication(&first, &second),
            Err("Matrix Multiplication is not possible".to_string())
        );
    }
    #[test]
    fn multiplication_second_failure() {
        let first = [[1, 2, 3].to_vec(), [4, 5, 6].to_vec(), [7, 8, 9].to_vec()].to_vec();
        let second = [[1, 2].to_vec(), [3, 4].to_vec()].to_vec();
        assert_eq!(
            matrix_multiplication(&first, &second),
            Err("Matrix Multiplication is not possible".to_string())
        );
    }
}

use log::*;
#[derive(Debug, PartialEq)]
/// Matrix structure
pub struct Matrix {
    pub element: [[i32; 3]; 3],
}
impl Matrix {
    /// matrix_multiplication method is used to calculate multiplication of two matrix
    ///
    /// #Arguments
    ///
    /// first_matrix:- first_matrix is of type Matrix
    /// second_matrix:- second_matrix is of type Matrix
    ///
    /// #Return
    ///
    /// Return Matrix type
    pub fn matrix_multiplication(first_matrix: Matrix, second_matrix: Matrix) -> Matrix {
        info!("Calculating multiplication of two matrix");
        let mut result = Matrix {
            element: [[0, 0, 0], [0, 0, 0], [0, 0, 0]],
        };
        for index_1 in 0..3 {
            for index_2 in 0..3 {
                for index_3 in 0..3 {
                    result.element[index_1][index_2] += first_matrix.element[index_1][index_3]
                        * second_matrix.element[index_3][index_2];
                }
            }
        }
        info!("Multiplication of two matrix is returned");
        result
    }
   /* /// print method is used to print result matrix in row-column form
    ///
    /// #Arguments
    ///
    /// self:- self represents the instance of Matrix structure
    ///
    /// #Return
    ///
    /// No Return
    pub fn print(self) {
        for index_1 in 0..3 {
            for index_2 in 0..3 {
                println!("{} ", self.element[index_1][index_2]);
            }
            println!(" ");
        }
    }*/
}

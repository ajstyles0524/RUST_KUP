use log::*;
/// matrix_multiplication method is used to calculate multiplication of two matrix
///
/// #Arguments
///
/// first_matrix:- first_matrix is a reference of Vec<Vec<i32>>
/// second_matrix:- second_matrix is a reference of Vec<Vec<i32>>
///
/// #Return
///
/// No Return
pub fn matrix_multiplication(
    first_matrix: &[Vec<i32>],
    second_matrix: &[Vec<i32>],
) -> Result<Vec<Vec<i32>>, String> {
    if first_matrix[0].len() != second_matrix.len() {
        return Err("Matrix Multiplication is not possible".to_string());
    }
    let mut result: Vec<Vec<i32>> = Vec::new();
    let l_1 = first_matrix.len();
    let l_2 = second_matrix.len();
    for (index_1, _item) in first_matrix.iter().enumerate().take(l_1)  {
        let mut vec: Vec<i32> = Vec::new();
        for index_2 in 0..second_matrix[0].len() {
            let mut sum = 0;
            for (index_3, _index) in second_matrix.iter().enumerate().take(l_2){
                sum += first_matrix[index_1][index_3] * second_matrix[index_3][index_2];
            }
            vec.push(sum);
        }
        result.push(vec);
    }
    info!("Multiplication of two matrix is returned successfully");
    Ok(result)
}

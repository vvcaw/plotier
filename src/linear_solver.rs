pub fn gaussian_elimination(mut matrix: Vec<Vec<f32>>) -> Vec<f32> {
    let matrix_length = matrix.len();

    (0..matrix_length).for_each(|column_index| {
        ((column_index + 1)..matrix_length).for_each(|row_index| {
            if matrix[column_index][column_index] == 0.0 {
                let mut k = column_index;

                // Search for a row where this cell is not 0
                while matrix[k][column_index] == 0.0 {
                   k += 1; 
                }

                // Swap rows
                matrix.swap(k, column_index);
            }

            // Eliminate coefficient
            let fac = -(matrix[row_index][column_index] / matrix[column_index][column_index]);
            (column_index..matrix_length + 1).for_each(|k| {
                matrix[row_index][k] += fac * matrix[column_index][k];
            });
        });
    });

    let mut solution: Vec<f32> = vec![];

    (0..matrix_length).rev().for_each(|column_index| {
        // Note that matrix_length always has to be the length of a row in gaussian elimination.
        solution.push(matrix[column_index][matrix_length]/matrix[column_index][column_index]);

        (0..(matrix_length - 1)).rev().for_each(|row_index| {
            matrix[row_index][matrix_length] -= solution.last().unwrap() * matrix[row_index][column_index];
        });
    });

    solution
}

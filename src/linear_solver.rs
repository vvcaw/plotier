/// Values passed to this function should be sorted in order.
pub fn generate_linear_system(points: &Vec<(f64, f64)>) -> Vec<Vec<f64>> {
    let polynomial_count = points.len() - 1;

    // Multiplicate by 4 as we have 4 coefficients.
    // Add one to row for result of each equation.
    let mut matrix = vec![vec![0.0; polynomial_count * 4 + 1]; polynomial_count * 4];

    for (equation_num, chunk) in points.windows(2).enumerate() {
        // Normal function boundaries
        let (x1, y1) = chunk[0];
        matrix[equation_num * 2][equation_num * 4] = x1 * x1 * x1;
        matrix[equation_num * 2][equation_num * 4 + 1] = x1 * x1;
        matrix[equation_num * 2][equation_num * 4 + 2] = x1;
        matrix[equation_num * 2][equation_num * 4 + 3] = 1.0;
        matrix[equation_num * 2][polynomial_count * 4] = y1;

        let (x2, y2) = chunk[1];
        matrix[equation_num * 2 + 1][equation_num * 4] = x2 * x2 * x2;
        matrix[equation_num * 2 + 1][equation_num * 4 + 1] = x2 * x2;
        matrix[equation_num * 2 + 1][equation_num * 4 + 2] = x2;
        matrix[equation_num * 2 + 1][equation_num * 4 + 3] = 1.0;
        matrix[equation_num * 2 + 1][polynomial_count * 4] = y2;

        // Only enter derivatives if we aren't in first or last iteration.
        if equation_num == 0 || equation_num == points.len() - 1 {
            continue;
        }

        let equation_num = equation_num - 1;

        // First derivative has to be equal at the point the two polynomials touch.
        matrix[polynomial_count * 2 + equation_num][equation_num * 4] = 3.0 * x1 * x1;
        matrix[polynomial_count * 2 + equation_num][equation_num * 4 + 1] = 2.0 * x1;
        matrix[polynomial_count * 2 + equation_num][equation_num * 4 + 2] = 1.0;

        matrix[polynomial_count * 2 + equation_num][equation_num * 4 + 4] = -3.0 * x1 * x1;
        matrix[polynomial_count * 2 + equation_num][equation_num * 4 + 5] = -2.0 * x1;
        matrix[polynomial_count * 2 + equation_num][equation_num * 4 + 6] = -1.0;

        // Second derivative has to be equal at the point the two polynomials touch.
        matrix[polynomial_count * 2 + polynomial_count - 1 + equation_num][equation_num * 4] =
            6.0 * x1;
        matrix[polynomial_count * 2 + polynomial_count - 1 + equation_num][equation_num * 4 + 1] =
            2.0;

        matrix[polynomial_count * 2 + polynomial_count - 1 + equation_num][equation_num * 4 + 4] =
            -6.0 * x1;
        matrix[polynomial_count * 2 + polynomial_count - 1 + equation_num][equation_num * 4 + 5] =
            -2.0;
    }

    // Boundary conditions for quadratic spline, a_1 + a_n = 0 => a_1 = -a_n => Quadratic Spline as other coefficients don't matter.
    matrix[polynomial_count * 4 - 1][polynomial_count * 4 - 4] = 1.0;
    matrix[polynomial_count * 4 - 2][0] = 1.0;

    matrix
}

pub fn gaussian_elimination(mut matrix: Vec<Vec<f64>>) -> Vec<f64> {
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

    let mut solution: Vec<f64> = vec![];

    (0..matrix_length).rev().for_each(|column_index| {
        // Note that matrix_length always has to be the length of a row in gaussian elimination.
        solution.push(matrix[column_index][matrix_length] / matrix[column_index][column_index]);

        (0..(matrix_length - 1)).rev().for_each(|row_index| {
            matrix[row_index][matrix_length] -=
                solution.last().unwrap() * matrix[row_index][column_index];
        });
    });

    // As gaussian elimination is solved bottom to top, list has to be reversed.
    solution.reverse();
    solution
}

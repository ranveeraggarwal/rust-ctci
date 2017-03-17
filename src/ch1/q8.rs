#[cfg(test)]
fn zero_matrix(matrix: &mut Vec<Vec<i32>>) -> () {
	let rows: usize = matrix.len();
	if rows == 0 {
		return ();
	}
	let cols: usize = matrix[0].len();
	if cols == 0 {
		return ();
	}

	let mut is_top_row_zero: bool = false;
	let mut is_first_col_zero: bool = false;
	for j in 0..cols {
		if matrix[0][j] == 0 {
			is_top_row_zero = true;
		}
	}
	for i in 0..rows {
		if matrix[i][0] == 0 {
			is_first_col_zero = true;
		}
	}

	for j in 1..cols {
		for i in 0..rows {
			if matrix[i][j] == 0 {
				matrix[0][j] = 0;
				break;
			}
		}
	}
	for i in 1..rows {
		for j in 0..cols {
			if matrix[i][j] == 0 {
				matrix[i][0] = 0;
				break;
			}
		}
	}

	for j in 1..cols {
		if matrix[0][j] == 0 {
			for i in 1..rows {
				matrix[i][j] = 0;
			}
		}
	}
	for i in 1..rows {
		if matrix[i][0] == 0 {
			for j in 1..cols {
				matrix[i][j] = 0;
			}
		}
	}

	if is_first_col_zero {
		for i in 1..rows {
			matrix[i][0] = 0;
		}
	}
	if is_top_row_zero {
		for j in 1..cols {
			matrix[0][j] = 0;
		}
	}
}

#[cfg(test)]
mod q8_tests {

	use ch1::q8::zero_matrix;

	#[test]
	fn zero_matrix_four_three() {
		let mut mut_matrix: Vec<Vec<i32>> = Vec::new();
		mut_matrix.push([1, 3, 4].to_vec());
		mut_matrix.push([2, 0, 3].to_vec());
		mut_matrix.push([6, 1, 1].to_vec());
		mut_matrix.push([9, 0, 9].to_vec());

		let mut comp_matrix: Vec<Vec<i32>> = Vec::new();
		comp_matrix.push([1, 0, 4].to_vec());
		comp_matrix.push([0, 0, 0].to_vec());
		comp_matrix.push([6, 0, 1].to_vec());
		comp_matrix.push([0, 0, 0].to_vec());

		zero_matrix(&mut mut_matrix);

		assert_eq!(mut_matrix, comp_matrix);
	}

	#[test]
	fn zero_matrix_one_one() {
		let mut mut_matrix: Vec<Vec<i32>> = Vec::new();
		mut_matrix.push([1].to_vec());

		let mut comp_matrix: Vec<Vec<i32>> = Vec::new();
		comp_matrix.push([1].to_vec());

		zero_matrix(&mut mut_matrix);

		assert_eq!(mut_matrix, comp_matrix);
	}

	#[test]
	fn zero_matrix_four_three_all_zero() {
		let mut mut_matrix: Vec<Vec<i32>> = Vec::new();
		mut_matrix.push([0, 3, 1].to_vec());
		mut_matrix.push([2, 0, 3].to_vec());
		mut_matrix.push([6, 1, 0].to_vec());
		mut_matrix.push([9, 0, 1].to_vec());

		let mut comp_matrix: Vec<Vec<i32>> = Vec::new();
		comp_matrix.push([0, 0, 0].to_vec());
		comp_matrix.push([0, 0, 0].to_vec());
		comp_matrix.push([0, 0, 0].to_vec());
		comp_matrix.push([0, 0, 0].to_vec());

		zero_matrix(&mut mut_matrix);

		assert_eq!(mut_matrix, comp_matrix);
	}
}
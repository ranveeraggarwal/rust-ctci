#[cfg(test)]
// A square matrix is assumed.
fn rotate(matrix: &mut Vec<Vec<i32>>) -> () {
    let size: usize = matrix.len();
    for i in 0..(size/2) {
        let start: usize = i;
        let end: usize = size-i-1;
        for j in start..end {
            let temp: i32 = matrix[i][j];
            matrix[i][j] = matrix[j][size-i-1];
            matrix[j][size-i-1] = matrix[size-i-1][size-j-1];
            matrix[size-i-1][size-j-1] = matrix[size-j-1][i];
            matrix[size-j-1][i] = temp;
        }
    }
}

#[cfg(test)]
mod q7_tests {

    use ch1::q7::rotate;

    #[test]
    fn rotate_matrix_twice() {
        let mut mut_vec = vec![vec![1; 4]; 4];
        let mut rot_vec = vec![vec![1; 4]; 4];
        for i in 0..4 {
            for j in 1..5 {
                mut_vec[i][j-1] = (i+j) as i32;
                rot_vec[4-j][3-i] = (i+j) as i32;
            }
        }
        rotate(&mut mut_vec);
        rotate(&mut mut_vec);
        assert_eq!(mut_vec, rot_vec);
    }

    #[test]
    fn rotate_single_matrix() {
        let mut mut_vec = vec![vec![1; 1]; 1];
        let rot_vec = vec![vec![1; 1]; 1];
        rotate(&mut mut_vec);
        rotate(&mut mut_vec);
        assert_eq!(mut_vec, rot_vec);
    }
}
/// Returns the number of inversions using the method described in Shilov §1.31:
/// the number of pairs where a larger value comes before a smaller one.
pub fn inversions(perm: &[usize]) -> usize {
    let mut count = 0;
    for i in 0..perm.len() {
        for j in i + 1..perm.len() {
            if perm[i] > perm[j] {
                count += 1;
            }
        }
    }
    count
}

/// Returns the sign of the term using the method described in Shilov §1.31:
/// 1 if the number of inversions is even, or -1 if odd.
pub fn sign(perm: &[usize]) -> i32 {
    if inversions(perm).is_multiple_of(2) {
        1
    } else {
        -1
    }
}

/// Returns the permutation (1, 2, …, n), which has no inversions.
pub fn identity(n: usize) -> Vec<usize> {
    let mut v = Vec::new();
    for i in 1..=n {
        v.push(i);
    }
    v
}

/// Returns all n! permutations of 1, …, n, as used in Shilov §1.31.
pub fn permutations(n: usize) -> Vec<Vec<usize>> {
    if n == 0 {
        vec![vec![]]
    } else {
        let smaller = permutations(n - 1);
        let mut result = Vec::new();
        for base in &smaller {
            for g in 0..=base.len() {
                let mut permutation = base.clone();
                permutation.insert(g, n);
                result.push(permutation);
            }
        }
        result
    }
}

/// An error type for determinant operations.
#[derive(Debug, PartialEq)]
pub enum DeterminantError {
    /// Some row’s length isn’t the number of rows.
    NotSquare,
    /// No rows at all. Shilov only defines matrices of order n >= 1.
    Empty,
}

/// Compute the determinant of an n-th order square matrix, as in Shilov §1.31, formula (6).
pub fn det(m: &[Vec<f64>]) -> Result<f64, DeterminantError> {
    if m.is_empty() {
        return Err(DeterminantError::Empty);
    }
    for row in m {
        if row.len() != m.len() {
            return Err(DeterminantError::NotSquare);
        }
    }

    let mut sum = 0.0;
    for alpha in permutations(m.len()) {
        let mut term = f64::from(sign(&alpha));
        for j in 0..alpha.len() {
            term *= m[alpha[j] - 1][j];
        }
        sum += term;
    }
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Shilov §1.31: N(2, 1, 4, 3) = 2
    #[test]
    fn inversions_of_2143_is_2() {
        assert_eq!(inversions(&[2, 1, 4, 3]), 2);
    }

    /// Shilov §1.31: N(4, 3, 1, 2) = 5
    #[test]
    fn inversions_of_4312_is_5() {
        assert_eq!(inversions(&[4, 3, 1, 2]), 5);
    }

    /// In-order test: N(1, 2, 3, 4) = 0
    #[test]
    fn inversions_of_1234_is_0() {
        assert_eq!(inversions(&[1, 2, 3, 4]), 0);
    }

    /// Shilov §1.31: N(2, 1, 4, 3) = 2 is even, so the sign is positive.
    #[test]
    fn sign_of_2143_is_pos() {
        assert_eq!(sign(&[2, 1, 4, 3]), 1);
    }

    /// Shilov §1.31: N(4, 3, 1, 2) = 5 is odd, so the sign is negative.
    #[test]
    fn sign_of_4312_is_neg() {
        assert_eq!(sign(&[4, 3, 1, 2]), -1);
    }

    /// In-order test: N(1, 2, 3, 4) = 0 is even, so the sign is positive.
    #[test]
    fn sign_of_1234_is_pos() {
        assert_eq!(sign(&[1, 2, 3, 4]), 1);
    }

    /// identity(4) equals vec![1, 2, 3, 4].
    #[test]
    fn identity_of_4_is_1234() {
        assert_eq!(identity(4), vec![1, 2, 3, 4]);
    }

    /// identity(0) equals vec![].
    #[test]
    fn identity_of_0_is_empty() {
        assert_eq!(identity(0), vec![]);
    }

    /// N(identity(5)) = 0 is even, so the sign is positive.
    #[test]
    fn sign_of_identity_of_5_is_pos() {
        assert_eq!(sign(&identity(5)), 1);
    }

    /// permutations(0) has length 1.
    #[test]
    fn permutations_of_0_has_length_1() {
        assert_eq!(permutations(0).len(), 1);
    }

    /// permutations(3) has length 6.
    #[test]
    fn permutations_of_3_has_length_6() {
        assert_eq!(permutations(3).len(), 6);
    }

    /// permutations(4) has length 24.
    #[test]
    fn permutations_of_4_has_length_24() {
        assert_eq!(permutations(4).len(), 24)
    }

    /// Every result of permutations(3) has length 3.
    #[test]
    fn permutations_of_3_has_results_of_length_3() {
        for result in permutations(3) {
            assert_eq!(result.len(), 3)
        }
    }

    /// The 6 results of permutations(3) are all different.
    #[test]
    fn permutations_of_3_are_distinct() {
        let perms = permutations(3);
        for i in 0..perms.len() {
            for j in i + 1..perms.len() {
                assert_ne!(perms[i], perms[j])
            }
        }
    }

    /// The signs of all permutations(4) add up to 0.
    #[test]
    fn signs_of_permutations_of_4_sum_to_0() {
        let mut sum = 0;
        for result in permutations(4) {
            sum += sign(&result);
        }
        assert_eq!(sum, 0)
    }

    /// True when `a` and `b` differ by less than 1e-12.
    fn close_f64(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-12
    }

    /// Order 1: the determinant of [[7]] is 7.
    #[test]
    fn det_of_order_1_is_its_element() {
        assert!(close_f64(det(&[vec![7.0]]).unwrap(), 7.0));
    }

    /// Order 2: the determinant of [[1, 2], [3, 4]] is -2,
    /// using the formula a11·a22 − a21·a12 in Shilov §1.31.
    #[test]
    fn det_of_order_2_example_is_minus_2() {
        assert!(close_f64(
            det(&[vec![1.0, 2.0], vec![3.0, 4.0]]).unwrap(),
            -2.0
        ));
    }

    /// Create an example 3x3 matrix: [[2, 0, 1], [1, 3, 2], [1, 1, 2]].
    fn example_3x3() -> Vec<Vec<f64>> {
        vec![
            vec![2.0, 0.0, 1.0],
            vec![1.0, 3.0, 2.0],
            vec![1.0, 1.0, 2.0],
        ]
    }

    /// Create an example 4x4 matrix: [[2, 0, 1, 3], [1, 3, 2, 1], [1, 1, 2, 3], [3, 2, 0, 2]].
    fn example_4x4() -> Vec<Vec<f64>> {
        vec![
            vec![2.0, 0.0, 1.0, 3.0],
            vec![1.0, 3.0, 2.0, 1.0],
            vec![1.0, 1.0, 2.0, 3.0],
            vec![3.0, 2.0, 0.0, 2.0],
        ]
    }

    /// Order 3: [[2, 0, 1], [1, 3, 2], [1, 1, 2]] is 6,
    /// using the formula in Shilov §1.31, page 6.
    #[test]
    fn det_of_order_3_example_is_6() {
        assert!(close_f64(det(&example_3x3()).unwrap(), 6.0));
    }

    /// The 3×3 identity matrix (1 on the diagonal, 0 elsewhere) has determinant 1.
    /// Only the in-order permutation gives a nonzero term.
    #[test]
    fn det_of_3x3_identity_is_1() {
        assert!(close_f64(
            det(&[
                vec![1.0, 0.0, 0.0],
                vec![0.0, 1.0, 0.0],
                vec![0.0, 0.0, 1.0]
            ])
            .unwrap(),
            1.0
        ));
    }

    /// Error case: an empty matrix gives Err(Empty).
    #[test]
    fn det_of_empty_matrix_is_error() {
        assert_eq!(det(&[]), Err(DeterminantError::Empty));
    }

    /// Error case: [[1, 2], [3]] gives Err(NotSquare).
    #[test]
    fn det_of_non_square_matrix_is_error() {
        assert_eq!(
            det(&[vec![1.0, 2.0], vec![3.0]]),
            Err(DeterminantError::NotSquare)
        );
    }

    /// Error case: [[1, 2], []] gives Err(NotSquare).
    #[test]
    fn det_of_one_empty_row_is_not_square() {
        assert_eq!(
            det(&[vec![1.0, 2.0], vec![]]),
            Err(DeterminantError::NotSquare)
        );
    }

    /// Error case: [[]] (one row, which is empty) gives Err(NotSquare).
    /// It has rows, so it passes the empty check; its row length 0 is not 1.
    #[test]
    fn det_of_single_empty_row_is_not_square() {
        assert_eq!(det(&[vec![]]), Err(DeterminantError::NotSquare));
    }

    /// Error case: [[1, 2, 3], [4, 5, 6]] (2 rows, 3 columns) gives Err(NotSquare).
    #[test]
    fn det_of_2x3_matrix_is_not_square() {
        assert_eq!(
            det(&[vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]),
            Err(DeterminantError::NotSquare)
        );
    }

    /// Returns the transposed matrix.
    fn transpose(m: &[Vec<f64>]) -> Vec<Vec<f64>> {
        let rows = m.len();
        let cols = m[0].len();
        let mut result = vec![vec![0.0; rows]; cols];
        for i in 0..rows {
            for j in 0..cols {
                result[j][i] = m[i][j];
            }
        }
        result
    }

    /// Transpose of [[1, 2, 3], [4, 5, 6]] is [[1, 4], [2, 5], [3, 6]].
    /// Exact comparison: transpose only copies values, with no arithmetic.
    #[test]
    fn transpose_of_2x3_matrix_is_3x2() {
        assert_eq!(
            transpose(&[vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]),
            vec![vec![1.0, 4.0], vec![2.0, 5.0], vec![3.0, 6.0]]
        );
    }

    /// §1.41: det(&transpose(&example_3x3())) is close to det(&example_3x3()), i.e. 6.
    #[test]
    fn det_of_transpose_equals_det_3x3() {
        assert!(close_f64(
            det(&transpose(&example_3x3())).unwrap(),
            det(&example_3x3()).unwrap()
        ));
    }

    /// §1.41: det(&transpose(&example_4x4())) is close to det(&example_4x4()), i.e. 12.
    #[test]
    fn det_of_transpose_equals_det_4x4() {
        assert!(close_f64(
            det(&transpose(&example_4x4())).unwrap(),
            det(&example_4x4()).unwrap()
        ));
    }

    /// Swap columns of a matrix (0-based).
    fn swap_columns(m: &[Vec<f64>], a: usize, b: usize) -> Vec<Vec<f64>> {
        let mut result = m.to_vec();
        for row in &mut result {
            row.swap(a, b);
        }
        result
    }

    /// Swapping columns 0 and 2 of [[1, 2, 3], [4, 5, 6]] gives [[3, 2, 1], [6, 5, 4]].
    /// Exact comparison: swap_columns only moves values, with no arithmetic.
    #[test]
    fn swap_columns_0_and_2_of_2x3() {
        assert_eq!(
            swap_columns(&[vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]], 0, 2),
            vec![vec![3.0, 2.0, 1.0], vec![6.0, 5.0, 4.0]]
        );
    }

    /// §1.42, neighbouring columns: swap columns 0 and 1 of example_3x3(). det is close to −6.
    #[test]
    fn swap_neighbouring_columns_flips_sign() {
        assert!(close_f64(
            det(&swap_columns(&example_3x3(), 0, 1)).unwrap(),
            -det(&example_3x3()).unwrap()
        ));
    }

    /// §1.42, far-apart columns: swap columns 0 and 3 of example_4x4(). det is close to −12.
    #[test]
    fn swap_far_apart_columns_flips_sign() {
        assert!(close_f64(
            det(&swap_columns(&example_4x4(), 0, 3)).unwrap(),
            -det(&example_4x4()).unwrap()
        ));
    }

    /// §1.43: a 3×3 with two equal columns has det close to 0.
    #[test]
    fn matrix_with_equal_columns_has_det_0() {
        assert!(close_f64(
            det(&[
                vec![0.0, 1.0, 0.0],
                vec![3.0, 3.0, 3.0],
                vec![1.0, 2.0, 1.0]
            ])
            .unwrap(),
            0.0
        ));
    }

    /// Returns a copy of matrix m with its column j replaced with col.
    fn with_column(m: &[Vec<f64>], j: usize, col: &[f64]) -> Vec<Vec<f64>> {
        let mut result = m.to_vec();
        for i in 0..result.len() {
            result[i][j] = col[i];
        }
        result
    }

    /// Returns a combined vector λb + μc.
    fn combine(lambda: f64, b: &[f64], mu: f64, c: &[f64]) -> Vec<f64> {
        let mut result = vec![0.0; b.len()];
        for i in 0..b.len() {
            result[i] = lambda * b[i] + mu * c[i];
        }
        result
    }

    /// with_column of [[1, 2], [3, 4]], column 1 replaced by [9, 8], is [[1, 9], [3, 8]].
    /// Exact comparison: with_column only copies values, with no arithmetic.
    #[test]
    fn with_column_2x2() {
        assert_eq!(
            with_column(&[vec![1.0, 2.0], vec![3.0, 4.0]], 1, &[9.0, 8.0]),
            vec![vec![1.0, 9.0], vec![3.0, 8.0]]
        );
    }

    /// combine(2.0, &[1.0, 2.0], 3.0, &[0.0, 1.0]) is [2, 7].
    /// Exact comparison: small integer products and sums are exact in f64.
    #[test]
    fn combine_2x2() {
        assert_eq!(combine(2.0, &[1.0, 2.0], 3.0, &[0.0, 1.0]), vec![2.0, 7.0]);
    }

    /// §1.44: D is linear in column 1: λ = 2.5, μ = −1.
    #[test]
    fn det_is_linear_in_a_column() {
        let m = example_3x3();
        let lambda = 2.5;
        let b = vec![1.0, 2.0, 3.0];
        let mu = -1.0;
        let c = vec![4.0, 5.0, 6.0];
        assert!(close_f64(
            det(&with_column(&m, 1, &combine(lambda, &b, mu, &c))).unwrap(),
            lambda * det(&with_column(&m, 1, &b)).unwrap()
                + mu * det(&with_column(&m, 1, &c)).unwrap()
        ));
    }

    /// §1.45: a common factor of a column comes outside: 3 times column 0 gives 3D.
    #[test]
    fn common_factor_of_a_column_comes_outside() {
        let m = example_3x3();
        let mut scaled_column = vec![0.0; m.len()];
        for i in 0..m.len() {
            scaled_column[i] = 3.0 * m[i][0];
        }
        assert!(close_f64(
            det(&with_column(&m, 0, &scaled_column)).unwrap(),
            3.0 * det(&m).unwrap()
        ));
    }

    /// §1.46: a column of zeros gives D = 0.
    #[test]
    fn zero_column_gives_det_0() {
        let m = example_3x3();
        assert!(close_f64(det(&with_column(&m, 2, &[0.0; 3])).unwrap(), 0.0));
    }
}

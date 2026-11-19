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
}

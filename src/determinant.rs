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

    /// Shilov §1.31: N(2, 1, 4, 3) = 2 is even, so the sign is 1.
    #[test]
    fn sign_of_2143_is_1() {
        assert_eq!(sign(&[2, 1, 4, 3]), 1);
    }

    /// Shilov §1.31: N(4, 3, 1, 2) = 5 is odd, so the sign is -1.
    #[test]
    fn sign_of_4312_is_neg_1() {
        assert_eq!(sign(&[4, 3, 1, 2]), -1);
    }

    /// In-order test: N(1, 2, 3, 4) = 0 is even, so the sign is 1.
    #[test]
    fn sign_of_1234_is_1() {
        assert_eq!(sign(&[1, 2, 3, 4]), 1);
    }
}

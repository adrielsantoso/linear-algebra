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
}

/// A complex number `re + i*im`, with both parts stored as `f64` (Shilov §1.12c).
#[derive(Debug, PartialEq)]
pub struct Complex {
    /// The real part.
    pub re: f64,
    /// The imaginary part.
    pub im: f64,
}

/// An error type for complex number operations.
#[derive(Debug, PartialEq)]
pub enum ComplexError {
    /// Returned by `recip` and `div` when the number to invert is exactly zero.
    DivisionByZero,
}

impl Complex {
    /// Builds the complex number `re + i*im`.
    pub fn new(re: f64, im: f64) -> Self {
        Complex { re, im }
    }

    /// Returns the negative `-self`, negating both real and imaginary parts.
    pub fn neg(&self) -> Complex {
        Complex {
            re: -self.re,
            im: -self.im,
        }
    }

    /// Returns the sum `self + other`, adding real and imaginary parts using the sum rule in Shilov §1.12c.
    pub fn add(&self, other: &Complex) -> Complex {
        Complex {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }

    /// Returns the difference `self - other`, computed as `self + (-other)`.
    pub fn sub(&self, other: &Complex) -> Complex {
        self.add(&other.neg())
    }

    /// Returns the product `self * other`, multiplying real and imaginary parts using the product rule in Shilov §1.12c.
    pub fn mul(&self, other: &Complex) -> Complex {
        Complex {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        }
    }

    /// Returns the reciprocal `1/self`, computed as `1/(re + i*im) = re/(re^2 + im^2) - i*im/(re^2 + im^2)` if re² + im² is not 0.0.
    pub fn recip(&self) -> Result<Complex, ComplexError> {
        let denom = self.re * self.re + self.im * self.im;
        if denom == 0.0 {
            return Err(ComplexError::DivisionByZero);
        }
        Ok(Complex {
            re: self.re / denom,
            im: -self.im / denom,
        })
    }

    /// Returns the quotient `self / other`, computed as `self * (1/other)`.
    pub fn div(&self, other: &Complex) -> Result<Complex, ComplexError> {
        let recip = other.recip()?;
        Ok(self.mul(&recip))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `new` stores both parts: z = 3 + 4i.
    #[test]
    fn complex_stores_real_and_imaginary_parts() {
        let z = Complex::new(3.0, 4.0);
        assert!((z.re - 3.0).abs() < 1e-12);
        assert!((z.im - 4.0).abs() < 1e-12);
    }

    /// True when both parts of `a` and `b` differ by less than 1e-12.
    fn close(a: &Complex, b: &Complex) -> bool {
        (a.re - b.re).abs() < 1e-12 && (a.im - b.im).abs() < 1e-12
    }

    /// Axiom 1: a + b = b + a
    #[test]
    fn addition_is_commutative() {
        let a = Complex::new(0.1, -2.5);
        let b = Complex::new(0.2, 4.0);

        assert!(close(&a.add(&b), &b.add(&a)));
    }

    /// Axiom 2: (a + b) + c = a + (b + c)
    #[test]
    fn addition_is_associative() {
        let a = Complex::new(0.1, -2.5);
        let b = Complex::new(0.2, 4.0);
        let c = Complex::new(0.3, 0.7);

        let left = (a.add(&b)).add(&c);
        let right = a.add(&b.add(&c));
        assert!(close(&left, &right));
    }

    /// Axiom 3: 0 + a = a
    #[test]
    fn zero_plus_a_is_a() {
        let zero = Complex::new(0.0, 0.0);
        let a = Complex::new(0.1, -2.5);

        assert!(close(&zero.add(&a), &a));
    }

    /// Axiom 4: a + (-a) = 0
    #[test]
    fn a_plus_neg_a_is_zero() {
        let a = Complex::new(0.1, -2.5);
        let zero = Complex::new(0.0, 0.0);

        assert!(close(&a.add(&a.neg()), &zero));
    }

    /// -(-a) = a
    #[test]
    fn neg_of_neg_a_is_a() {
        let a = Complex::new(0.1, -2.5);
        assert!(close(&a.neg().neg(), &a));
    }

    /// (5 + 3i) - (2 + i) = 3 + 2i.
    #[test]
    fn sub_subtracts_parts() {
        let a = Complex::new(5.0, 3.0);
        let b = Complex::new(2.0, 1.0);

        assert!(close(&a.sub(&b), &Complex::new(3.0, 2.0)));
    }

    /// Axiom 5: a * b = b * a
    #[test]
    fn multiplication_is_commutative() {
        let a = Complex::new(0.1, -2.5);
        let b = Complex::new(0.2, 4.0);

        assert!(close(&a.mul(&b), &b.mul(&a)));
    }

    /// Axiom 6: (a * b) * c = a * (b * c)
    #[test]
    fn multiplication_is_associative() {
        let a = Complex::new(0.1, -2.5);
        let b = Complex::new(0.2, 4.0);
        let c = Complex::new(0.3, 0.7);

        let left = (a.mul(&b)).mul(&c);
        let right = a.mul(&b.mul(&c));
        assert!(close(&left, &right));
    }

    /// Axiom 7: 1 * a = a
    #[test]
    fn one_times_a_is_a() {
        let one = Complex::new(1.0, 0.0);
        let a = Complex::new(0.1, -2.5);

        assert!(close(&one.mul(&a), &a));
    }

    /// Axiom 8: a * (1 / a) = 1, for a != 0
    #[test]
    fn a_times_recip_is_one() {
        let a = Complex::new(0.1, -2.5);
        let one = Complex::new(1.0, 0.0);

        assert!(close(&a.mul(&a.recip().unwrap()), &one));
    }

    /// Error case: recip(0) gives Err(DivisionByZero).
    #[test]
    fn recip_of_zero_is_error() {
        let zero = Complex::new(0.0, 0.0);
        assert_eq!(zero.recip(), Err(ComplexError::DivisionByZero));
    }

    /// Error case: a / 0 gives Err(DivisionByZero).
    #[test]
    fn a_div_zero_is_error() {
        let a = Complex::new(0.1, -2.5);
        let zero = Complex::new(0.0, 0.0);
        assert_eq!(a.div(&zero), Err(ComplexError::DivisionByZero));
    }

    /// 1 / (1 / a) = a, for a != 0
    #[test]
    fn recip_of_recip_is_a() {
        let a = Complex::new(0.1, -2.5);
        assert!(close(&a.recip().unwrap().recip().unwrap(), &a));
    }

    /// (2 + 4i) / (1 + i) = 3 + i.
    #[test]
    fn div_divides_parts() {
        let a = Complex::new(2.0, 4.0);
        let b = Complex::new(1.0, 1.0);

        assert!(close(&a.div(&b).unwrap(), &Complex::new(3.0, 1.0)));
    }

    /// Axiom 9: a * (b + c) = a * b + a * c
    #[test]
    fn multiplication_distributes_over_addition() {
        let a = Complex::new(0.1, -2.5);
        let b = Complex::new(0.2, 4.0);
        let c = Complex::new(0.3, 0.7);

        let left = a.mul(&b.add(&c));
        let right = a.mul(&b).add(&a.mul(&c));
        assert!(close(&left, &right));
    }

    /// i^2 = -1
    #[test]
    fn i_squared_is_minus_one() {
        let i = Complex::new(0.0, 1.0);
        let minus_one = Complex::new(-1.0, 0.0);

        assert!(close(&i.mul(&i), &minus_one));
    }
}

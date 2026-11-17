/// A complex number `re + i*im`, with both parts stored as `f64` (Shilov §1.12c).
pub struct Complex {
    /// The real part.
    pub re: f64,
    /// The imaginary part.
    pub im: f64,
}

impl Complex {
    /// Builds the complex number `re + i*im`.
    pub fn new(re: f64, im: f64) -> Self {
        Complex { re, im }
    }

    /// Returns the real part.
    pub fn re(&self) -> f64 {
        self.re
    }

    /// Returns the imaginary part.
    pub fn im(&self) -> f64 {
        self.im
    }

    /// Returns the sum `self + other`, adding real and imaginary parts
    /// using the sum rule in Shilov §1.12c.
    pub fn add(&self, other: &Complex) -> Complex {
        Complex {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }

    /// Returns the product `self * other`, multiplying real and imaginary parts
    /// using the product rule in Shilov §1.12c.
    pub fn mul(&self, other: &Complex) -> Complex {
        Complex {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `new` stores both parts: z = 3 + 4i.
    #[test]
    fn complex_stores_real_and_imaginary_parts() {
        let z: Complex = Complex::new(3.0, 4.0);
        assert!((z.re() - 3.0).abs() < 1e-12);
        assert!((z.im() - 4.0).abs() < 1e-12);
    }

    /// True when both parts of `a` and `b` differ by less than 1e-12.
    fn close(a: &Complex, b: &Complex) -> bool {
        (a.re - b.re).abs() < 1e-12 && (a.im - b.im).abs() < 1e-12
    }

    /// Axiom 1: a + b = b + a
    #[test]
    fn addition_is_commutative() {
        let a: Complex = Complex::new(0.1, -2.5);
        let b: Complex = Complex::new(0.2, 4.0);

        assert!(close(&a.add(&b), &b.add(&a)));
    }

    /// Axiom 2: (a + b) + c = a + (b + c)
    #[test]
    fn addition_is_associative() {
        let a: Complex = Complex::new(0.1, -2.5);
        let b: Complex = Complex::new(0.2, 4.0);
        let c: Complex = Complex::new(0.3, 0.7);

        let left: Complex = (a.add(&b)).add(&c);
        let right: Complex = a.add(&b.add(&c));
        assert!(close(&left, &right));
    }

    /// Axiom 3: 0 + a = a
    #[test]
    fn zero_plus_a_is_a() {
        let zero: Complex = Complex::new(0.0, 0.0);
        let a: Complex = Complex::new(0.1, -2.5);

        assert!(close(&zero.add(&a), &a));
    }

    /// Axiom 5: a * b = b * a
    #[test]
    fn multiplication_is_commutative() {
        let a: Complex = Complex::new(0.1, -2.5);
        let b: Complex = Complex::new(0.2, 4.0);

        assert!(close(&a.mul(&b), &b.mul(&a)));
    }

    /// Axiom 6: (a * b) * c = a * (b * c)
    #[test]
    fn multiplication_is_associative() {
        let a: Complex = Complex::new(0.1, -2.5);
        let b: Complex = Complex::new(0.2, 4.0);
        let c: Complex = Complex::new(0.3, 0.7);

        let left: Complex = (a.mul(&b)).mul(&c);
        let right: Complex = a.mul(&b.mul(&c));
        assert!(close(&left, &right));
    }

    /// Axiom 7: 1 * a = a
    #[test]
    fn one_times_a_is_a() {
        let one: Complex = Complex::new(1.0, 0.0);
        let a: Complex = Complex::new(0.1, -2.5);

        assert!(close(&one.mul(&a), &a));
    }

    /// Axiom 9: a * (b + c) = a * b + a * c
    #[test]
    fn multiplication_distributes_over_addition() {
        let a: Complex = Complex::new(0.1, -2.5);
        let b: Complex = Complex::new(0.2, 4.0);
        let c: Complex = Complex::new(0.3, 0.7);

        let left: Complex = a.mul(&b.add(&c));
        let right: Complex = a.mul(&b).add(&a.mul(&c));
        assert!(close(&left, &right));
    }

    /// i^2 = -1
    #[test]
    fn i_squared_is_minus_one() {
        let i: Complex = Complex::new(0.0, 1.0);
        let minus_one: Complex = Complex::new(-1.0, 0.0);

        assert!(close(&i.mul(&i), &minus_one));
    }
}

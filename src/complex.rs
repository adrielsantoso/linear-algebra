pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Complex {
    pub fn new(re: f64, im: f64) -> Self {
        Complex { re, im }
    }

    pub fn re(&self) -> f64 {
        self.re
    }

    pub fn im(&self) -> f64 {
        self.im
    }

    pub fn add(&self, other: &Complex) -> Complex {
        Complex {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complex_stores_real_and_imaginary_parts() {
        // Create the complex number z = 3 + 4i
        let z: Complex = Complex::new(3.0, 4.0);
        assert!((z.re() - 3.0).abs() < 1e-12);
        assert!((z.im() - 4.0).abs() < 1e-12);
    }

    /// True when both parts of `a` and `b` differ by less than 1e-12.
    fn close(a: &Complex, b: &Complex) -> bool {
        (a.re - b.re).abs() < 1e-12 && (a.im - b.im).abs() < 1e-12
    }

    #[test]
    fn addition_is_commutative() {
        // Axiom 1: a + b = b + a
        let a: Complex = Complex::new(0.1, -2.5);
        let b: Complex = Complex::new(0.2, 4.0);

        assert!(close(&a.add(&b), &b.add(&a)));
    }

    #[test]
    fn addition_is_associative() {
        // Axiom 2: (a + b) + c = a + (b + c)
        let a: Complex = Complex::new(0.1, -2.5);
        let b: Complex = Complex::new(0.2, 4.0);
        let c: Complex = Complex::new(0.3, 0.7);

        let left: Complex = a.add(&b).add(&c);
        let right: Complex = a.add(&b.add(&c));
        assert!(close(&left, &right));
    }

    #[test]
    fn zero_plus_a_is_a() {
        // Axiom 3: 0 + a = a
        let zero: Complex = Complex::new(0.0, 0.0);
        let a: Complex = Complex::new(0.1, -2.5);

        assert!(close(&zero.add(&a), &a));
    }
}

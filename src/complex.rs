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
}

pub mod solution {
    use std::ops::{Add, AddAssign};
    use std::fmt;
    use std::hash::{Hash, Hasher};
    use std::cmp::Ordering;

    /// Represents a complex number with real and imaginary parts
    #[derive(Debug, Clone, Copy, Default, PartialEq)]
    pub struct ComplexNumber {
        real: f64,
        imag: f64,
    }

    #[derive(Debug, PartialEq)]
    pub enum ComplexNumberError {
        ImaginaryNotZero,
    }

    impl ComplexNumber {
        /// Creates a new complex number
        pub fn new(real: f64, imag: f64) -> Self {
            ComplexNumber { real, imag }
        }

        /// Creates a complex number from the real part
        pub fn from_real(real: f64) -> Self {
            ComplexNumber { real, imag: 0.0 }
        }

        /// Returns the real part
        pub fn real(&self) -> f64 {
            self.real
        }

        /// Returns the imaginary part
        pub fn imag(&self) -> f64 {
            self.imag
        }

        /// Returns the complex number as a tuple (real, imag)
        pub fn to_tuple(&self) -> (f64, f64) {
            (self.real, self.imag)
        }

        /// Calculates the modulus of the complex number
        pub fn modulus(&self) -> f64 {
            (self.real * self.real + self.imag * self.imag).sqrt()
        }
    }

    // Display trait implementation
    impl fmt::Display for ComplexNumber {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{} + {}i", self.real, self.imag)
        }
    }

    // Add trait implementation for ComplexNumber + ComplexNumber
    impl Add for ComplexNumber {
        type Output = Self;

        fn add(self, other: Self) -> Self::Output {
            ComplexNumber::new(self.real + other.real, self.imag + other.imag)
        }
    }

    // Add trait implementation for ComplexNumber + f64
    impl Add<f64> for ComplexNumber {
        type Output = Self;

        fn add(self, other: f64) -> Self::Output {
            ComplexNumber::new(self.real + other, self.imag)
        }
    }

    // Add trait implementation for ComplexNumber + &ComplexNumber
    impl Add<&ComplexNumber> for ComplexNumber {
        type Output = Self;

        fn add(self, other: &ComplexNumber) -> Self::Output {
            ComplexNumber::new(self.real + other.real, self.imag + other.imag)
        }
    }

    // Add trait implementation for &ComplexNumber + &ComplexNumber
    impl Add<&ComplexNumber> for &ComplexNumber {
        type Output = ComplexNumber;

        fn add(self, other: &ComplexNumber) -> Self::Output {
            ComplexNumber::new(self.real + other.real, self.imag + other.imag)
        }
    }

    // AddAssign trait implementation
    impl AddAssign for ComplexNumber {
        fn add_assign(&mut self, other: Self) {
            self.real += other.real;
            self.imag += other.imag;
        }
    }

    // TryInto<f64> trait implementation
    impl TryInto<f64> for ComplexNumber {
        type Error = ComplexNumberError;

        fn try_into(self) -> Result<f64, Self::Error> {
            if self.imag == 0.0 {
                Ok(self.real)
            } else {
                Err(ComplexNumberError::ImaginaryNotZero)
            }
        }
    }

    // From<f64> trait implementation
    impl From<f64> for ComplexNumber {
        fn from(real: f64) -> Self {
            ComplexNumber::from_real(real)
        }
    }

    // Eq trait implementation (automatically derived from PartialEq)
    impl Eq for ComplexNumber {}

    // PartialOrd trait implementation
    impl PartialOrd for ComplexNumber {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    // Ord trait implementation
    impl Ord for ComplexNumber {
        fn cmp(&self, other: &Self) -> Ordering {
            let self_mod = self.modulus();
            let other_mod = other.modulus();
            self_mod.total_cmp(&other_mod)
        }
    }

    // AsRef<f64> trait implementation
    impl AsRef<f64> for ComplexNumber {
        fn as_ref(&self) -> &f64 {
            &self.real
        }
    }

    // AsMut<f64> trait implementation
    impl AsMut<f64> for ComplexNumber {
        fn as_mut(&mut self) -> &mut f64 {
            &mut self.real
        }
    }

    // Hash trait implementation
    impl Hash for ComplexNumber {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.real.to_bits().hash(state);
            self.imag.to_bits().hash(state);
        }
    }
}

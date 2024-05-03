use nalgebra::{Complex, matrix, Matrix2};

/// Identity and Pauli Matrices
pub const HBAR: f64 = 1.0545718e-34;
pub const I: Matrix2<Complex<f64>> = matrix![Complex::new(1.0, 0.0), Complex::new(0.0, 0.0); Complex::new(0.0, 0.0), Complex::new(0.0, 1.0)];
pub const PAULI_X: Matrix2<Complex<f64>> = matrix![Complex::new(0.0, 0.0), Complex::new(1.0, 1.0); Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)];
pub const PAULI_Y: Matrix2<Complex<f64>> = matrix![Complex::new(0.0, 0.0), Complex::new(0.0, -1.0); Complex::new(0.0, 1.0), Complex::new(0.0, 0.0)];
pub const PAULI_Z: Matrix2<Complex<f64>> = matrix![Complex::new(1.0, 0.0), Complex::new(0.0, 0.0); Complex::new(0.0, 0.0), Complex::new(0.0, -1.0)];

/// Time evolution operator U(t)
pub fn time_evo_operator(h: &Matrix2<Complex<f64>>, t: f64) -> Matrix2<Complex<f64>> {
    let a = Complex::new(0.0, -1.0) * Complex::from(t);
    let exponent = scale_matrix(h, a);

    exponent.exp()
}

fn scale_matrix(mat: &Matrix2<Complex<f64>>, scalar: Complex<f64>) -> Matrix2<Complex<f64>> {
    mat.map(|x| scalar * x)
}

/// Generalized Spin-1/2 Hamiltonian
pub struct Hamiltonian {
    operator: Matrix2<Complex<f64>>,
}

impl Hamiltonian {
    pub fn new(theta: f64, phi: f64) -> Hamiltonian {
        let omega_x = Complex::from(theta.cos() * phi.sin());
        let omega_y = Complex::from(theta.sin() * phi.sin());
        let omega_z = Complex::from(phi.cos());

        let scale = Complex::from(0.5 * HBAR);

        let h: Matrix2<Complex<f64>> = scale_matrix(&(scale_matrix(&PAULI_X, omega_x) + scale_matrix(&PAULI_Y, omega_y) + scale_matrix(&PAULI_Z, omega_z)), scale);
        Hamiltonian { operator: h }
    }
}

/// Bloch sphere vector to represent a pure or mixed state
pub struct BlochVector {
    pub theta: f64,
    pub phi: f64,
    pub r: f64,
}

impl BlochVector {
    pub fn new(theta: f64, phi: f64, r: f64) -> BlochVector {
        if r > 1.0 {
            panic!("Not a valid state! Magnitude must be less than or equal to one")
        }

        BlochVector {
            theta: theta,
            phi: phi,
            r: r,
        }
    }

    pub fn coordinates(&self) -> (f64, f64, f64) {
        let x = self.phi.sin() * self.theta.cos() * self.r;
        let y = self.phi.sin() * self.theta.sin() * self.r;
        let z = self.phi.cos() * self.r;

        (x,y,z)
    }
}

/// Qubit represented on a bloch sphere.
 pub struct Qubit {
    pub initial_state: BlochVector,
    pub density_matrix: Matrix2<Complex<f64>>,
}

impl Qubit {
    pub fn new(a: BlochVector) -> Qubit {
        let (a_x, a_y, a_z) = a.coordinates();
        // Convert to complex numbers
        let (x, y, z) = (Complex::from(a_x), Complex::from(a_y), Complex::from(a_z));

        let density: Matrix2<Complex<f64>> = scale_matrix(&(I + scale_matrix(&PAULI_X, x) + scale_matrix(&PAULI_Y, y) + scale_matrix(&PAULI_Z, z)), Complex::from(0.5));
        Qubit {
            initial_state: a,
            density_matrix: density,
        }
    }

    pub fn evolve(&mut self, h: &Hamiltonian, t: f64) {
        let u = time_evo_operator(&h.operator, t);
        let u_dagger = u.transpose().conjugate();

        self.density_matrix = u * self.density_matrix * u_dagger;
    }

    pub fn measure(&self, a: &Matrix2<Complex<f64>>) -> Result<f64, String> {
        let measured = self.density_matrix * a;
        let measurement = measured.trace();
        if measurement.im != 0.0 {
            Err("Invalid measurement operator!".to_owned())
        } else {
            Ok(measurement.re)
        }
    }
} 
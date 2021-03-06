
use num::complex::Complex;
use num::traits::Zero;

pub const MAX_ITERATIONS: u32 = 1000;

pub fn get_iterations(x: f64, y: f64) -> u32 {
    let mut z : Complex<f64> = Zero::zero();
	let c : Complex<f64> = Complex::new(x, y);

	let mut iterations : u32 = 0;

	while iterations < MAX_ITERATIONS {
		z = z*z + c;

		if z.norm() > 2.0 {
			break;
		}

		iterations += 1;
	}

	iterations
}

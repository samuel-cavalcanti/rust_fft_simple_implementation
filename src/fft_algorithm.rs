use core::f64;
use std::f64::consts::PI;

use num::complex::Complex64;

 // Fast Fourier Transform
 pub fn fft(signal: &Vec<Complex64>) -> Vec<Complex64> {
    fft_algorithm(signal, 1.0)
}
// Fast inverse Fourier Transform
pub fn ifft(signal: &Vec<Complex64>) -> Vec<Complex64> {
    fft_algorithm(signal, -1.0)
}
/*
Guilde to Competitive Programming - Antti Laaksonen 
Learning and Improving Algorithms Through Contests
Second Edition,  pag 198 - Fast fourier Transform
*/
fn fft_algorithm(signal: &Vec<Complex64>, d: f64) -> Vec<Complex64> {
    let signal_size = signal.len();

    let mut result_signal = vec![Complex64::new(0.0, 0.0); signal.len()];

    for k in 0..signal_size {
        let mut b = 0;
        for z in (0..).map(|z| 1 << z).take_while(|&z| z < signal_size) {
            b *= 2;

            if (k & z) != 0 {
                b += 1
            }
        }

        result_signal[b] = signal[k];
    }

    for m in (1..).map(|m| 1 << m).take_while(|&m| m <= signal_size) {
        let omega_m = Complex64::new(0.0, d * 2.0 * PI / (m as f64)).exp();

        for k in (0..signal_size).step_by(m) {
            let mut omega = Complex64::new(1.0, 0.0);
            for j in 0..m / 2 {
                let u = result_signal[k + j];
                let t = omega * result_signal[k + j + m / 2];

                result_signal[k + j] = u + t;
                result_signal[k + j + m / 2] = u - t;

                omega = omega_m * omega;
            }
        }
    }

    if d == -1.0 {
        let complex_signal_size = Complex64::new(signal_size as f64, 0.0);

        result_signal = result_signal
            .iter()
            .map(|complex_value| complex_value / complex_signal_size)
            .collect();
    }

    result_signal
}
use num::complex::Complex64;

use crate::fft_algorithm::{fft,ifft};

fn new_complex_vector_from_float_vector(vector: &Vec<f64>, size: usize) -> Vec<Complex64> {
    let vector_size = vector.len();
    (0..size)
        .map(|index| {
            if index >= vector_size {
                Complex64::new(0.0, 0.0)
            } else {
                Complex64::new(vector[index], 0.0)
            }
        })
        .collect()
}

fn find_new_size_power_of_two(bigger_exp: usize) -> usize {
    let mut new_size: usize = 1 << 4;

    for i in 3..bigger_exp {
        if new_size < bigger_exp {
            new_size = 1 << i;
        } else {
            break;
        }
    }

    new_size
}

pub fn product_of_polynomials(f_x: &Vec<f64>, g_x: &Vec<f64>) -> Vec<f64> {
    let f_x_size = f_x.len();
    let g_x_size = g_x.len();

    let bigger_exp = f_x_size + g_x_size;

    let new_size = find_new_size_power_of_two(bigger_exp);

    let f = new_complex_vector_from_float_vector(f_x, new_size);

    let g = new_complex_vector_from_float_vector(g_x, new_size);

    let t_f = fft(&f);

    let t_g = fft(&g);

    let t_p = (0..t_g.len()).map(|i| t_f[i] * t_g[i]).collect();

    let p = ifft(&t_p);

    let real_part: Vec<f64> = p
        .iter()
        .map(|complex_value| complex_value.re.round())
        .filter(|&real_value| real_value != 0.0)
        .collect();

    real_part
}
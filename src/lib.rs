mod fft_algorithm;
mod product_of_polynomials;



#[cfg(test)]
mod tests {

    use super::product_of_polynomials::product_of_polynomials;

    #[test]
    fn test_product_of_polynomials() {
        // f(x) =  3 + 2x
        let f_x = vec![3.0, 2.0];

        // g(x) =  1 + 5x
        let g_x = vec![1.0, 5.0];

        let p_x = product_of_polynomials(&f_x, &g_x);

        // p(x) = g(x)*f(x) = 3 + 17x + 10x²
        assert_eq!(p_x, vec![3.0, 17.0, 10.0]);

        // f(x) =  5 + 2x + 2x²
        let f_x = vec![5.0, 2.0, 2.0];

        // g(x) =  1 + 5x
        let g_x = vec![1.0, 5.0];

        let p_x = product_of_polynomials(&f_x, &g_x);

        // p(x) = g(x)*f(x) = 5 + 27x + 10x + 0x³
        assert_eq!(p_x, vec![5.0, 27.0, 12.0, 10.0]);

        // f(x) =  5 + 3x + x² + 0x³
        let f_x = vec![5.0, 3.0, 1.0, 0.0];

        // f(x) =  5 + 5x + 0x² + 0x³
        let g_x = vec![5.0, 5.0, 0.0, 0.0];

        let p_x = product_of_polynomials(&f_x, &g_x);

        // p(x) = g(x)*f(x) = 25 + 40x + 15x² + 0x³
        assert_eq!(p_x, vec![25.0, 40.0, 20.0, 5.0]);
    }
}

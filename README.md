# Uma Simples Implementação da Transformada Rápida de Fourier em Rust

Com o objetivo de compreender melhor a transformada de Fourier
resolvi implementar de maneria simples o seu simples algoritmo:
_Fast Fourier Transform_ (FFT)
e para testar a sua implementação, utilizei em uma das suas famosas aplicações, que é o cálculo do produto de 2 polinômios.  
Exemplo:
```rust
        // f(x) =  3 + 2x
        let f_x = vec![3.0, 2.0];

        // g(x) =  1 + 5x
        let g_x = vec![1.0, 5.0];

        let p_x = product_of_polynomials(&f_x, &g_x);

        // p(x) = g(x)*f(x) = 3 + 17x + 10x²
        assert_eq!(p_x, vec![3.0, 17.0, 10.0]);
```
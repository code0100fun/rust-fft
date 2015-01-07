#![feature(slicing_syntax)]
#![feature(macro_rules)]
extern crate num;
extern crate test;

use test::Bencher;
use num::complex::{Complex};
use std::num::Float;
use std::num::FloatMath;
use std::f64::consts;
use std::rand::{thread_rng, Rng};

#[macro_export]
macro_rules! complex_vec(
    ($($real:expr, $imaginary:expr),*) => ({
        let mut _temp = ::std::vec::Vec::new();
        $(
            let c = Complex::new($real, $imaginary);
            _temp.push(c);
         )*
        _temp
    });
    ($($e:expr),+,) => (vec!($($e),+))
);

#[macro_export]
macro_rules! assert_complex_vec_eq(
    ($expected:expr, $actual:expr, $tolerance:expr) => (
        for (i, a) in $actual.iter().enumerate() {
            let e = $expected[i];
            let diff = e - *a;
            assert!(Float::abs(diff.re) < $tolerance && Float::abs(diff.im) < $tolerance);
        }
    );
);

pub struct FFT;

impl FFT {
    pub fn new() -> FFT {
        let fft = FFT;
        fft
    }

    pub fn execute_real(&self, vec: Vec<f64>) -> Vec<Complex<f64>> {
        let length = vec.len();
        if length == 1 {
            return complex_vec!(vec[0],0f64);
        }
        let mut i = 0i32;
        let (even, odd): (Vec<f64>,Vec<f64>) = vec.into_iter().partition(|_: &f64| {
            let ret = i % 2 == 0;
            i +=  1;
            return ret;
        });

        let mut fft_even = self.execute_real(even);
        let mut fft_odd = self.execute_real(odd);

        for e in fft_even.clone().iter() {
            fft_even.push(*e);
        }
        for o in fft_odd.clone().iter() {
            fft_odd.push(*o);
        }

        let mut output = Vec::new();

        for i in range(0, length) {
            let c = fft_even[i] + self.omega(-(i as f64), length as f64) * fft_odd[i];
            output.push(c);
        }

        return output;
    }

    pub fn omega(&self, k: f64, n: f64) -> Complex<f64> {
        // e^(i2πk/n) = cos(2πk/n) + sin(2πk/n)i
        let theta = (consts::PI_2 * k) / n;
        Complex::new(FloatMath::cos(theta), FloatMath::sin(theta))
    }
}

#[test]
fn real_fft_impulse_origin() {
    let input = vec!(
        1f64,
        0f64,
        0f64,
        0f64,
        0f64,
        0f64,
        0f64,
        0f64
    );
    let actual = FFT::new().execute_real(input);
    let expected = complex_vec!(
        1f64, 0f64,
        1f64, 0f64,
        1f64, 0f64,
        1f64, 0f64,
        1f64, 0f64,
        1f64, 0f64,
        1f64, 0f64,
        1f64, 0f64
    );
    assert_complex_vec_eq!(expected, actual, 1.0E-6f64);
}

#[test]
fn real_fft() {
    let input = vec!(
        0f64,
        1f64,
        2f64,
        3f64,
        4f64,
        5f64,
        6f64,
        7f64
    );
    let actual = FFT::new().execute_real(input);
    let expected = complex_vec!(
        28f64, 0f64,
        -4f64, 9.656854f64,
        -4f64, 4f64,
        -4f64, 1.656854f64,
        -4f64, 0f64,
        -4f64, -1.656854f64,
        -4f64, -4f64,
        -4f64, -9.656854f64
    );
    assert_complex_vec_eq!(expected, actual, 1.0E-6f64);
}


#[bench]
fn large_real_fft(b: &mut Bencher) {
    static BENCH_SIZE: uint = 256 * 1024;
    let mut rng = thread_rng();
    let input = rng.gen_iter::<f64>().take(BENCH_SIZE).collect::<Vec<f64>>();
    b.iter(|| {
        FFT::new().execute_real(input.clone())
    });
}

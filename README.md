# Rust-FFT

A pure Rust [Fast Fourier Transform](http://en.wikipedia.org/wiki/Fast_Fourier_transform) library.

No optimizations have been made yet so this implementation is still very slow compared to [FFTW3](http://www.fftw.org/).

# Installation
Add it to your `Cargo.toml`:

```toml
[dependencies.fft]
git = "git://github.com/code0100fun/rust-fft"
```

## Run Tests

```
cargo test
```

### Run Benchmarks

Benchmaring is very slow for now (~20 sec). A single run of 256K samples is actually only 0.28 sec but Rust bench runs ~50 times.

```
cargo bench
```

## TODO

- complex input vector
- [butterfly](http://www.cmlab.csie.ntu.edu.tw/cml/dsp/training/coding/transform/fft.html)
- parallelize
- inline instead of recursive

# rust_FFT_experiments
 initial experiments with rustfft crate

install rust dev evironment, the cd to where cargo.toml file is, then run

cargo run --release

for release (faster/optimised) version or 

cargo run   

...to build and run debug (slower version)

cargo build --release
cargo build     (debug)
cargo clean     (removes all temp files)

Example Output  (release only)
===============================
Intel(R) Core(TM) i5-4590 CPU @ 3.30GHz, 
Windows 10 (64-bit) 16.0 GB RAM

E:\rust\rust_FFT_experiments>cargo run --release
warning: crate `myFFT` should have a snake case name
  |
  = help: convert the identifier to snake case: `my_fft`
  = note: `#[warn(non_snake_case)]` on by default

warning: `myFFT` (bin "myFFT") generated 1 warning
    Finished release [optimized] target(s) in 0.22s
     Running `target\release\myFFT.exe`
fft.process() on 512point fft 1024 iterations and took 0 msec
fft.process() on 1024point fft 1024 iterations and took 2 msec
fft.process() on 2048point fft 1024 iterations and took 4 msec
fft.process() on 4096point fft 1024 iterations and took 9 msec
fft.process() on 8192point fft 1024 iterations and took 57 msec
fft.process() on 16384point fft 1024 iterations and took 56 msec
fft.process() on 32768point fft 1024 iterations and took 155 msec
fft.process() on 65536point fft 1024 iterations and took 330 msec


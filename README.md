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

fft.process() on 512point fft 1024 iterations and took 0 msec

fft.process() on 1024point fft 1024 iterations and took 2 msec

fft.process() on 2048point fft 1024 iterations and took 4 msec

fft.process() on 4096point fft 1024 iterations and took 9 msec

fft.process() on 8192point fft 1024 iterations and took 57 msec

fft.process() on 16384point fft 1024 iterations and took 56 msec

fft.process() on 32768point fft 1024 iterations and took 155 msec

fft.process() on 65536point fft 1024 iterations and took 330 msec


With test frequency set to 128 the plots png files come out like this:
![test_fft_output](https://user-images.githubusercontent.com/50658445/236908002-7050678c-e211-4399-82ed-cab9abb18952.png)
![test_input](https://user-images.githubusercontent.com/50658445/236908003-4db1c93c-7d90-4de6-8c8b-bc62c469333c.png)

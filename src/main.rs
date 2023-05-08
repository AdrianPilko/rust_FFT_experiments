fn test_fft() {
    // generate a waveform at 1234Hz with sample frequency of 10000Hz, then fft and check the peak is in right cell
    use rustfft::{num_complex::Complex, FftPlanner};
    use std::vec;
    use waver::Wave;
    let fft_size = 4096;
    let freq: f32 = 1234.0;
    let samp_freq: f32 = 10000.0;
    let debug = false;

    let test_sig = Wave {
        sample_rate: samp_freq,
        frequency: freq,
        amplitude: 1.0,
        ..Default::default()
    };

    // get  4096 samples from test_sig, this seams extremely long winded but couldn't find a
    // way to simply get 4096 complex values from the waveform, rustdoc ?!

    let mut test_sig_output = vec![Complex { re: 0.0, im: 0.0 }; fft_size];
    let extracted_wave: Vec<f32> = test_sig.iter().take(fft_size).collect();
    for i in 0..test_sig_output.len() {
        test_sig_output[i].re = extracted_wave[i];
        test_sig_output[i].im = 0.0; // signal is real only
    }

    if debug == true {
        for i in 0..test_sig_output.len() {
            // probably a better way to iterate over vector, but might not be as readalbe!
            println!("{} ,  {} , ", i, test_sig_output[i]);
        }
    }

    let mut planner = FftPlanner::<f32>::new();
    let fft = planner.plan_fft_forward(fft_size);

    // this is an in place fft so output is written over the input
    fft.process(&mut test_sig_output);

    // print magnitude (using norm()) and check for peak
    let cell_expected_peak = ((fft_size as f32) / (samp_freq as f32)) * freq as f32;
    let mut max_so_far: f32 = 0.0;
    let mut cell_at_max = 0;
    for i in 0..test_sig_output.len()
    // probably a better way to iterate over vector, but might not be as readalbe!
    {
        if max_so_far < test_sig_output[i].norm() {
            max_so_far = test_sig_output[i].norm();
            cell_at_max = i;
        }
        if debug == true {
            println!("{} ,  {} ", i, test_sig_output[i].norm());
        }
    }
    // due to expected cell lying between cells the peak cell is not exactly as expect, would have
    // to interpolate the two or three peak cells to get exact frequency in fft output
    println!(
        "peak found at cell {} versus expected {}",
        cell_at_max, cell_expected_peak
    );
}

fn benchmark() {
    // Perform a forward FFT of various sizes
    use rustfft::FftPlanner; // we may have to use non avx if cpu does not support it
    use rustfft::{num_complex::Complex, FftPlannerAvx};
    use std::time::Instant;

    for fft_size in [512, 1024, 2048, 4096, 8192, 16384, 32768, 65536] {
        let mut buffer = vec![Complex { re: 0.0, im: 0.0 }; fft_size];
        let number_of_fft = 1024;

        if let Ok(mut planner_avx) = FftPlannerAvx::<f32>::new() {
            let fft_avx = planner_avx.plan_fft_forward(fft_size);
            let now = Instant::now();

            for _count in 1..=number_of_fft {
                fft_avx.process(&mut buffer);
            }

            let elapsed_time = now.elapsed();
            println!(
                "fft.process() on {}point fft {} iterations and took {} msec",
                fft_size,
                number_of_fft,
                elapsed_time.as_millis()
            );
        } else {
            let mut planner = FftPlanner::<f32>::new(); // FftPlannerAvx is also valailable if CPU supports it!
            let fft = planner.plan_fft_forward(fft_size);
            let now = Instant::now();
            for _count in 1..=number_of_fft {
                fft.process(&mut buffer);
            }
            let elapsed_time = now.elapsed();
            println!(
                "fft.process() on {}point fft {} iterations and took {} msec",
                fft_size,
                number_of_fft,
                elapsed_time.as_millis()
            );
        }
    }
}

fn main() {
    // run the benchmark timings on various fft lengths
    benchmark();

    // run a basic test on a generator signal
    test_fft();
}

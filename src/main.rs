fn main() {
  
    // Perform a forward FFT of size 32768
  use rustfft::{FftPlanner, num_complex::Complex};
  use std::time::Instant;
  
  for fft_size in [512,1024,2048,4096,8192,16384,32768,65536]
  {
    let mut planner = FftPlanner::<f32>::new();
    let fft = planner.plan_fft_forward(fft_size);
    let mut buffer = vec![Complex{ re: 0.0, im: 0.0 }; fft_size];
    
    let number_of_fft = 1024;
    
    let now = Instant::now();    
    
    for _count in 1..=number_of_fft
    {
      fft.process(&mut buffer);
    }
    
    let elapsed_time = now.elapsed();  
    println!("fft.process() on {}point fft {} iterations and took {} msec", fft_size, number_of_fft, elapsed_time.as_millis());
  }
}

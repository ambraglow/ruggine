mod fft;
mod graph;

use crate::{fft::complex::Sine, graph::Plot};
use anyhow::{Ok, Result};
use num_complex::Complex32;

fn main() -> Result<(), anyhow::Error> {
    // let random_vectors = RandComplex::random_vec(300, 1.0).unwrap();
    // let re: Vec<Complex64> = random_vectors
    //     .iter()
    //     .map(|item: &Complex64| Complex64::new(item.re().cos(), item.im().sin()))
    //     .collect();
    // //let re: Vec<f64> = re.iter().map(|x| x.re + x.im).collect();
    // println!("{:?}", re);

    // let mut sigout: Vec<Complex64> = Vec::new();
    // let mut sum = Complex64::new(0.0, 0.0);
    // for item in &re {
    //     sum += Complex64::new(item.re() + item.im(), item.re());
    //     sigout.push(sum);
    //     //println!("{:?}", sigout);
    // }

    let sample_rate = 44000;
    let frequency = 1000f32;

    let sine2 = Sine::new(frequency, sample_rate);
    let mut sine2: Vec<f32> = sine2.take(10).collect();
    println!("{sine2:?}");

    // sine2 = sine2.iter().map(|a| a * 10 as f32).collect();
    // println!("{:?}", sine2.len());

    sine2.clone().plot(Some("sine".to_string()))?;
    let dft: Vec<Complex32> = fft::dft::dft(&mut sine2, Some(false)).unwrap();
    println!("{dft:?} length: {:?}", dft.len());

    dft.plot(Some("dft".to_string()))?;

    Ok(())
}

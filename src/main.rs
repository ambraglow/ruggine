mod fft;
mod graph;

use crate::{
    fft::{
        complex::{RandComplex, Sine},
        dft::{Dft, FourierTransform},
    },
    graph::Plot,
};
use anyhow::{Ok, Result};

const SAMPLE_RATE: u32 = 500;
const NR_SAMPLES: usize = 5000;

fn main() -> Result<(), anyhow::Error> {
    let mut dft = FourierTransform::new();

    let sine1 = Sine::new(200f32, SAMPLE_RATE);
    // let sine2 = Sine::new(100f32, SAMPLE_RATE);
    let sine1: Vec<f32> = sine1.take(NR_SAMPLES).collect();
    // let sine2: Vec<f32> = sine2.take(NR_SAMPLES).collect();

    // dft.signal.append(&mut sine1);

    dft.signal = RandComplex::random_vec(NR_SAMPLES, 0.1).unwrap();
    // dft.signal = sine1.iter().zip(sine1).map(|(a, b)| a + b).collect();
    dft.signal = sine1.iter().zip(dft.signal).map(|(a, b)| a + b).collect();

    dft.signal.clone().plot(Some("signal".to_string()))?;
    dft.dft();
    // dft.fft();

    dft.plot(Some("dft".to_string()))?;

    Ok(())
}

use anyhow::{Ok, Result};
use num_complex::{Complex32, ComplexFloat};
use std::{f32::consts::TAU, vec};

pub struct FourierTransform {
    pub bins: Vec<Complex32>,
    pub signal: Vec<f32>,
}

impl FourierTransform {
    pub fn new() -> Self {
        FourierTransform {
            bins: Vec::new(),
            signal: Vec::new(),
        }
    }
}

#[allow(dead_code)]
pub trait Dft {
    fn simple_dft(&mut self, input: &mut Vec<f32>) -> Vec<Complex32>;
    fn dft(&mut self);
    fn fft(&mut self);
}

#[allow(dead_code)]
pub trait Window {
    // fn hamming();
    fn rectangular(&mut self, points: u32);
}

#[allow(dead_code)]
impl Window for FourierTransform {
    fn rectangular(&mut self, points: u32) {
        let mut magnitude: Vec<f32> = self.bins.iter().map(|a| a.abs()).collect();
    }
}

impl Dft for FourierTransform {
    fn simple_dft(&mut self, input: &mut Vec<f32>) -> Vec<Complex32> {
        let signal = input.len();
        let mut temp: Vec<Complex32> = vec![Complex32::default(); signal as usize];

        (0..signal)
            .flat_map(|frequency_bin| (0..signal).map(move |sample| (frequency_bin, sample)))
            .for_each(|(frequency_bin, sample)| {
                let angle: f32 = -TAU * frequency_bin as f32 * sample as f32 / signal as f32;
                let complex: Complex32 = Complex32::new(angle.cos(), angle.sin());
                temp[frequency_bin] += input[sample] * complex;
            });

        temp
    }

    /// Simple DFT
    fn dft(&mut self) {
        let signal = self.signal.len() as i32;
        self.bins = vec![Complex32::default(); signal as usize];

        (0..signal as usize)
            .flat_map(|frequency_bin| {
                (0..signal as usize).map(move |sample| (frequency_bin, sample))
            })
            .for_each(|(frequency_bin, sample)| {
                let angle: f32 = -TAU * frequency_bin as f32 * sample as f32 / signal as f32;
                let complex: Complex32 = Complex32::new(angle.cos(), angle.sin());
                self.bins[frequency_bin] += self.signal[sample] * complex;
            });
    }

    /// Incomplete radix-2 FFT implementation
    fn fft(&mut self) {
        let signal: usize = self.signal.len();
        self.bins = vec![Complex32::default(); signal as usize];

        let mut even: Vec<_> = vec![f32::default(); signal];
        let mut odd: Vec<_> = vec![f32::default(); signal];

        let even = self.simple_dft(&mut even);
        let odd = self.simple_dft(&mut odd);

        // let mut even = self.fft();
        // let mut odd = self.fft();

        for m in 0..signal / 2 {
            let m_alias = m % (signal / 2);
            let angle: f32 = -TAU * m as f32 / signal as f32;
            // let complex: Complex32 = Complex32::new(angle.cos(), angle.sin());
            self.bins[m as usize] = even[m_alias as usize] + angle * odd[m_alias as usize];
        }
    }
}

#[allow(dead_code)]
fn split(input: &mut FourierTransform) -> Result<(Vec<f32>, Vec<f32>)> {
    let (even_indices, odd_indices): (Vec<_>, Vec<_>) = input
        .signal
        .iter()
        .enumerate()
        .partition(|(i, _)| i % 2 == 0);

    let even: Vec<f32> = even_indices.into_iter().map(|(_, val)| *val).collect();
    let odd: Vec<f32> = odd_indices.into_iter().map(|(_, val)| *val).collect();

    Ok((even, odd))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Sine;

    const SAMPLE_RATE: u32 = 500;
    const NR_SAMPLES: usize = 1000;

    fn signal_helper() -> Result<FourierTransform> {
        let mut dft = FourierTransform::new();
        let sine1: Vec<f32> = Sine::new(200f32, SAMPLE_RATE).take(NR_SAMPLES).collect();

        dft.signal = sine1.iter().zip(dft.signal).map(|(a, b)| a + b).collect();

        Ok(dft)
    }

    #[test]
    fn running_dft() {
        let mut dft = signal_helper().unwrap();
        let signal_len: usize = dft.signal.len();

        dft.dft();
        assert_eq!(signal_len, dft.bins.len());
    }

    #[test]
    fn splitting() {
        let mut dft = signal_helper().unwrap();
        let signal_len: usize = dft.signal.len();

        let mut even: Vec<_> = vec![f32::default(); signal_len];
        let mut odd: Vec<_> = vec![f32::default(); signal_len];

        (even, odd) = split(&mut dft).unwrap();

        let size_of_odd: usize = odd.len();
        let size_of_even: usize = even.len();
        let tot_size: usize = size_of_even + size_of_odd;

        assert_eq!(signal_len, tot_size);
    }
}

// This is basically what happens in the iterator but as for loops, code kept for reference
// for frequency_bin in 0..signal {
//     let mut sum: Complex32 = Complex32::default();

//     for sample in 0..signal {
//         // if inverse.unwrap() {
//         //     let angle: f32 = TAU * a as f32 * b as f32 / size as f32;
//         // }
//         let angle: f32 = -TAU * frequency_bin as f32 * sample as f32 / signal as f32;
//         let complex: Complex32 = Complex32::new(angle.cos(), angle.sin());

//         sum += self.signal[sample as usize] * complex;

//         // if inverse.unwrap() {
//         //     sum.im /= size as f32;
//         //     sum.re /= size as f32;
//         // }
//         // println!("i: {a} j: {b} angle: {angle}");
//     }
//     self.bins.push(sum);
// }

// quick and dirty implementation of dft
// reference code from https://rosettacode.org/wiki/Discrete_Fourier_transform - C example
// pub fn dft(input: &mut Vec<f32>, inverse: Option<bool>) -> Option<Vec<Complex32>> {
//     let size = input.len() as i32;
//     let mut temp: Vec<Complex32> = Vec::new();

//     for a in 0..size {
//         let mut sum: Complex32 = Complex32::default();
//         for b in 0..size {
//             let angle: f32 = -TAU * a as f32 * b as f32 / size as f32;
//             let complex: Complex32 = Complex32::new(angle.cos(), angle.sin());

//             sum += input[b as usize] * complex;
//         }
//         temp.push(sum);
//     }
//     Some(temp)
// }

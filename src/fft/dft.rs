use num_complex::{Complex32, ComplexFloat};
use std::{f32::consts::TAU, vec};

use crate::SAMPLE_RATE;

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
    Some(temp)
}

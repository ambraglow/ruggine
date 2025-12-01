mod app;
mod fft;
mod graph;
mod sdr;

use crate::{
    fft::{
        complex::{RandComplex, Sine},
        dft::{Dft, FourierTransform},
    },
    graph::Plot,
};
use anyhow::{Ok, Result};
use tokio;

const SAMPLE_RATE: u32 = 1000;
const NR_SAMPLES: usize = 2000;

fn main() -> Result<()> {
    // Initialize tokio runtime for background tasks
    let _rt = tokio::runtime::Runtime::new().unwrap();

    let result = _rt.spawn(async {
        let mut dft = FourierTransform::new();

        let sine1 = Sine::new(200f32, SAMPLE_RATE);
        let sine2 = Sine::new(100f32, SAMPLE_RATE);
        let sine1: Vec<f32> = sine1.take(NR_SAMPLES).collect();
        let sine2: Vec<f32> = sine2.take(NR_SAMPLES).collect();

        // dft.signal.append(&mut sine1);

        // dft.signal = RandComplex::random_vec(NR_SAMPLES, 0.1).unwrap();
        dft.signal = sine1.iter().zip(sine2).map(|(a, b)| a + b).collect();
        dft.signal = sine1.iter().zip(dft.signal).map(|(a, b)| a + b).collect();

        dft.signal.clone().plot(Some("signal".to_string())).unwrap();
        dft.dft();
        // dft.fft();

        dft.plot(Some("dft".to_string())).unwrap();
    });

    let spawn_window = app::gui::App::display_window();

    match spawn_window {
        std::result::Result::Ok(v) => {
            println!("success! spawned window on main thread {v:?}")
        }
        std::result::Result::Err(e) => {
            println!("Error spawning window on main thread {e:?}")
        }
    }

    Ok(())
}

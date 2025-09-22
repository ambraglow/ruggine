use num_complex::Complex32;
use std::f32::consts::TAU;

// fn clean(input: &mut Complex64) {}

// quick and dirty implementation of dft :sob:
// reference code from https://rosettacode.org/wiki/Discrete_Fourier_transform - C example
pub fn dft(input: &mut Vec<f32>, inverse: Option<bool>) -> Option<Vec<Complex32>> {
    let N = input.len() as i32;
    // println!("{N:?}");
    let mut temp: Vec<Complex32> = Vec::new();
    let mut sum: Complex32 = Complex32::new(0f32, 0f32);

    for i in 0..N {
        for j in 0..N {
            if inverse.unwrap() {
                let angle: f32 = TAU * i as f32 * j as f32 / N as f32;
            }
            let angle: f32 = -TAU * i as f32 * j as f32 / N as f32;
            let complex: Complex32 = Complex32::new(angle.cos(), angle.sin());
            input.iter_mut().for_each(|f: &mut f32| sum += *f * complex);
            if inverse.unwrap() {
                sum.im /= N as f32;
                sum.re /= N as f32;
            }
            temp.push(sum);
            // println!("i: {i} j: {j} angle: {angle}");
        }
    }
    Some(temp)
}

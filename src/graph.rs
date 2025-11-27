use anyhow::Ok;
use num_complex::{Complex, Complex32, Complex64, ComplexFloat};
use plotters::{prelude::*, style::full_palette::BLUEGREY};

use crate::{SAMPLE_RATE, fft::dft::FourierTransform};

pub trait Plot<T>: Sized {
    fn plot(self, l: Option<T>) -> Result<(), anyhow::Error>;
}

impl Plot<String> for Vec<f32> {
    fn plot(self, l: Option<String>) -> Result<(), anyhow::Error> {
        let label = l.unwrap_or("".to_string());
        let filename = format!("./{}-plot.png", label);
        let root = BitMapBackend::new(&filename, (1024, 640)).into_drawing_area();
        root.fill(&WHITE)?;

        let y_max: i32 = self.iter().map(|&f| f as i32).max().unwrap();
        let y_min: i32 = self.iter().map(|&f| f as i32).min().unwrap();

        let mut chart = ChartBuilder::on(&root)
            .caption(label, ("sans-serif", 30).into_font())
            .margin(5)
            .x_label_area_size(50)
            .y_label_area_size(50)
            .build_cartesian_2d(-2.0f32..self.len() as f32, -2.0..y_max as f32 + 1.0f32)?;

        chart.configure_mesh().draw()?;

        chart
            .draw_series(LineSeries::new(
                self.into_iter().enumerate().map(|(x, y)| (x as f32, y)),
                &RED,
            ))?
            .label("plot")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x, y)], RED));

        chart
            .configure_series_labels()
            .background_style(&BLACK.mix(0.2))
            .border_style(&BLACK)
            .draw()?;

        root.present()?;

        Ok(())
    }
}

impl Plot<String> for FourierTransform {
    fn plot(self, l: Option<String>) -> Result<(), anyhow::Error> {
        let label = l.unwrap_or("".to_string());
        let filename = format!("./{}-plot.png", label);
        let root = BitMapBackend::new(&filename, (1024, 640)).into_drawing_area();
        root.fill(&WHITE)?;

        let mag: Vec<f32> = self.bins.iter().map(|a| a.abs()).collect();

        let mut sort = mag.clone();
        sort.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let max_y = *sort.iter().last().unwrap();

        let res = SAMPLE_RATE as f32 / self.bins.len() as f32;

        // println!("freq {:?}", * (SAMPLE_RATE as f32 / self.len() as f32);
        // mag.iter()
        //     .clone()
        //     .for_each(|f| println!("{:?}", f * (SAMPLE_RATE as f32 / self.len() as f32)));

        let mut chart = ChartBuilder::on(&root)
            .caption(label, ("sans-serif", 30).into_font())
            .margin(5)
            .x_label_area_size(50)
            .y_label_area_size(50)
            .build_cartesian_2d(
                0f32..(self.bins.len() as f32 * res),
                (0.0..max_y * 2.0).log_scale(),
            )?;

        chart.configure_mesh().draw()?;

        chart
            .draw_series(LineSeries::new(
                mag.into_iter()
                    .enumerate()
                    .map(|(x, y)| ((x as f32 * res), y)),
                &BLUEGREY,
            ))?
            .label("magnitude")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x, y)], RED));

        chart
            .configure_series_labels()
            .background_style(&BLACK.mix(0.2))
            .border_style(&BLACK)
            .draw()?;

        root.present()?;

        Ok(())
    }
}

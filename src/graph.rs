use anyhow::Ok;
use num_complex::{Complex, Complex32, Complex64};
use plotters::prelude::*;

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
            .build_cartesian_2d(-2.0f32..self.len() as f32, -1.0..y_max as f32 + 1.0f32)?;

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

impl Plot<String> for Vec<Complex64> {
    fn plot(self, l: Option<String>) -> Result<(), anyhow::Error> {
        let label = l.unwrap_or("".to_string());
        let filename = format!("./{}-plot.png", label);
        let root = BitMapBackend::new(&filename, (1024, 640)).into_drawing_area();
        root.fill(&WHITE)?;

        let real: Vec<f64> = self.iter().map(|item: &Complex64| item.re).collect();

        let im: Vec<f64> = self.iter().map(|item: &Complex64| item.im).collect();

        let y_max: i32 = im.iter().map(|&f| f as i32).max().unwrap();

        let mut chart = ChartBuilder::on(&root)
            .caption(label, ("sans-serif", 30).into_font())
            .margin(5)
            .x_label_area_size(50)
            .y_label_area_size(50)
            .build_cartesian_2d(-1.0..im.len() as f64, -1.0..y_max as f64 * 1.2)?;

        chart.configure_mesh().draw()?;

        // chart
        //     .draw_series(LineSeries::new(
        //         im.into_iter().enumerate().map(|(x, y)| (x as f64, y.abs())),
        //         &RED,
        //     ))?
        //     .label("imaginary")
        //     .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));

        // chart
        //     .draw_series(LineSeries::new(
        //         real.into_iter()
        //             .enumerate()
        //             .map(|(x, y)| (x as f64, y.abs())),
        //         &BLUE,
        //     ))?
        //     .label("real")
        //     .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE));

        chart
            .draw_series(LineSeries::new(
                self.into_iter()
                    .enumerate()
                    .map(|(x, y)| (x as f64, y.re.abs())),
                &RED,
            ))?
            .label("signal")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));

        chart.configure_mesh().draw()?;

        chart
            .configure_series_labels()
            .background_style(&BLACK.mix(0.2))
            .border_style(&BLACK)
            .draw()?;

        root.present()?;

        Ok(())
    }
}

impl Plot<String> for Vec<Complex32> {
    fn plot(self, l: Option<String>) -> Result<(), anyhow::Error> {
        let label = l.unwrap_or("".to_string());
        let filename = format!("./{}-plot.png", label);
        let root = BitMapBackend::new(&filename, (1024, 640)).into_drawing_area();
        root.fill(&WHITE)?;

        let sample_rate = 44000;
        let frequency = 1000f32;

        let mag: Vec<Complex32> = self
            .iter()
            .map(|a| Complex32::new(a.re.abs(), a.im.abs()))
            .collect();

        let real: Vec<f32> = self.iter().map(|item: &Complex32| item.re).collect();
        let im: Vec<f32> = self.iter().map(|item: &Complex32| item.im).collect();

        let y_max: i32 = mag.iter().map(|&f| f.im as i32).max().unwrap();
        // println!("y_max: {:?}", y_max);
        let dF = sample_rate / self.len();

        // println!("dF: {:?} len: {:?}", dF, real.len());

        let mut chart = ChartBuilder::on(&root)
            .caption(label, ("sans-serif", 30).into_font())
            .margin(5)
            .x_label_area_size(50)
            .y_label_area_size(50)
            .build_cartesian_2d(0f32..self.len() as f32 / 2f32, 0f32..y_max as f32)?;

        chart.configure_mesh().draw()?;

        chart
            .draw_series(LineSeries::new(
                mag.into_iter().enumerate().map(|(x, y)| (x as f32, y.im)),
                &RED,
            ))?
            .label("signal")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));

        // chart
        //     .draw_series(LineSeries::new(
        //         real.into_iter()
        //             .enumerate()
        //             .map(|(x, y)| (x as f32, y.abs())),
        //         &BLUE,
        //     ))?
        //     .label("real")
        //     .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE));

        // chart
        //     .draw_series(LineSeries::new(
        //         mag.into_iter().map(|(y)| (y.re, y.im)),
        //         &RED,
        //     ))?
        //     .label("signal")
        //     .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));

        chart.configure_mesh().draw()?;

        chart
            .configure_series_labels()
            .background_style(&BLACK.mix(0.2))
            .border_style(&BLACK)
            .draw()?;

        root.present()?;

        Ok(())
    }
}

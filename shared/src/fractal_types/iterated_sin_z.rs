use std::fmt::{Display, Error, Formatter};

use complex_math::Complex;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use serde::{Deserialize, Serialize};

use crate::{
    complementary_types::pixelintensity::PixelIntensity,
    fractal_implementation::{fractal::GetDatas, fractal_calcul::iterated_sin_z},
    messages::message::FragmentTask,
};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct IteratedSinZ {
    pub c: Complex,
}

impl GetDatas for IteratedSinZ {
    fn get_datas(&self, task: &FragmentTask) -> Vec<PixelIntensity> {
        let x_start = task.range.min.x;
        let x_end = task.range.max.x;
        let y_start = task.range.min.y;
        let y_end = task.range.max.y;

        let number_of_pixels = task.resolution.nx as u32 * task.resolution.ny as u32;

        let x_step = (x_end - x_start) / task.resolution.nx as f64;
        let y_step = (y_end - y_start) / task.resolution.ny as f64;

        let max_iteration = task.max_iteration;

        let datas: Vec<PixelIntensity> = (0..number_of_pixels)
            .into_par_iter() // Utilisation de rayon pour le traitement parallèle
            .map(|i| {
                let x = x_start + (i % task.resolution.ny as u32) as f64 * x_step;
                let y = y_start + (i / task.resolution.ny as u32) as f64 * y_step;

                let pixel_complexe = Complex::new(x, y);
                let fractal_result = iterated_sin_z(pixel_complexe, self.c, max_iteration);

                PixelIntensity::new(fractal_result.0, fractal_result.1)
            })
            .collect();

        datas
    }
}

impl Display for IteratedSinZ {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "IteratedSinZ")
    }
}

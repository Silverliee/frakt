use serde::{Deserialize, Serialize};

use crate::{
    client_calcul::libs::fractal_lib,
    messages::{complementary_types::pixelintensity::PixelIntensity, fragment_task::FragmentTask},
};

use super::super::super::complementary_types::complex::Complex;

pub trait GetDatas {
    fn get_datas(&self, task: &FragmentTask) -> Vec<PixelIntensity>;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JuliaDescriptor {
    pub divergence_threshold_square: f64,
    pub c: Complex,
}

impl GetDatas for JuliaDescriptor {
    fn get_datas(&self, task: &FragmentTask) -> Vec<PixelIntensity> {
        let x_start = task.range.min.x;
        let x_end = task.range.max.x;
        let y_start = task.range.min.y;
        let y_end = task.range.max.y;

        let x_step = ((&x_start - &x_end) / task.resolution.nx as f64).abs();
        let y_step = ((&y_start - &y_end) / task.resolution.ny as f64).abs();

        let mut pixel_intensity_vec: Vec<PixelIntensity> = Vec::new();

        let max_iteration = task.max_iteration;
        let mut x = x_start;
        let mut y = y_start;

        while y < y_end {
            while x < x_end {
                let pixel_complexe = Complex::new(x, y);
                let fractal_result = fractal_lib::julia(
                    pixel_complexe,
                    self.c,
                    self.divergence_threshold_square,
                    max_iteration,
                );
                pixel_intensity_vec.push(PixelIntensity::new(fractal_result.0, fractal_result.1));
                x += x_step;
            }
            x = x_start;
            y += y_step;
        }

        return pixel_intensity_vec;
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Mandelbrot {}

impl GetDatas for Mandelbrot {
    fn get_datas(&self, task: &FragmentTask) -> Vec<PixelIntensity> {
        let x_start = task.range.min.x;
        let x_end = task.range.max.x;
        let y_start = task.range.min.y;
        let y_end = task.range.max.y;

        let x_step = ((&x_start - &x_end) / task.resolution.nx as f64).abs();
        let y_step = ((&y_start - &y_end) / task.resolution.ny as f64).abs();

        let mut pixel_intensity_vec: Vec<PixelIntensity> = Vec::new();
        let max_iteration = task.max_iteration;

        let mut x = x_start;
        let mut y = y_start;

        while y < y_end {
            while x < x_end {
                let pixel_complexe = Complex::new(x, y);
                let fractal_result = fractal_lib::mandelbrot(pixel_complexe, max_iteration);
                pixel_intensity_vec.push(PixelIntensity::new(fractal_result.0, fractal_result.1));
                x += x_step;
            }
            x = x_start;
            y += y_step;
        }

        return pixel_intensity_vec;
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IteratedSinZ {
    c: Complex,
}

impl GetDatas for IteratedSinZ {
    fn get_datas(&self, task: &FragmentTask) -> Vec<PixelIntensity> {
        return Vec::new();
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FractalDescriptor {
    Julia(JuliaDescriptor),
    Mandelbrot(Mandelbrot),
    IteratedSinZ(IteratedSinZ),
}

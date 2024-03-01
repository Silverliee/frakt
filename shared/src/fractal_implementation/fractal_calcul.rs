//! # Fractal Library
//!
//! A library for generating fractal images using various algorithms.
//!
//! ## Algorithms
//!
//! The library provides implementations for the following fractal algorithms:
//!
//! - [Julia Set](#method.julia)
//! - [Mandelbrot Set](#method.mandelbrot)
//! - [Iterated Sin(z)](#method.iterated_sin_z)
//! - [Newton-Raphson (z^3)](#method.newton_raphson_z_3)
//! - [Newton-Raphson (z^4)](#method.newton_raphson_z_4)
//! - [Nova Newton-Raphson (z^3)](#method.nova_newton_raphson_z_3)
//! - [Nova Newton-Raphson (z^4)](#method.nova_newton_raphson_z_4)
//!
//! ## Color Generation
//!
//! The library also provides a color function for mapping a continuous parameter `t` to an RGB color.
//!
//! ## Image Creation
//!
//! You can create fractal images using the `create_image` function, which takes a `FragmentTask` and a vector of `PixelIntensity`.
//! The images are saved to the specified path, and the file name is generated randomly.
//!
//! # Examples
//!
//! ```rust
//! use complex_math::Complex;
//! use fractal_lib::{create_image, julia, PixelIntensity, FragmentTask};
//!
//! fn main() {
//!     // Example usage of fractal generation and image creation
//!     let z = Complex::new(0.0, 0.0);
//!     let c = Complex::new(-0.8, 0.156);
//!     let max_divergence = 1000.0;
//!     let max_iter = 200;
//!
//!     let fractal_task = FragmentTask {
//!         fractal: "julia".to_string(),
//!         resolution: (800, 600),
//!         // Other fields initialization...
//!     };
//!
//!     let mut pixel_intensity_vec = Vec::new();
//!
//!     // Populate pixel_intensity_vec with intensity values...
//!
//!     match create_image(&fractal_task, &pixel_intensity_vec, Some("./output/")) {
//!         Ok(_) => println!("Image created successfully."),
//!         Err(err) => eprintln!("Error creating image: {}", err),
//!     }
//! }
//! ```

use std::{f64::consts::PI, fs};

use complex_math::Complex;
use image::ImageError;
use rand::{thread_rng, Rng};

use crate::{complementary_types::pixelintensity::PixelIntensity, messages::message::FragmentTask};

use super::fractal::FractalDescriptor;

///Compute julia fractal value for given parameters
/// * `z` - The complex number to compute the julia fractal value for z
/// * `c` - The complex number to compute the julia fractal value for c
/// * `max_divergence` - The maximum divergence value
/// * `max_iter` - The maximum number of iterations
/// * Return: a tuple of two f32 values (zn, count)
pub fn julia(z: Complex, c: Complex, max_divergence: f64, max_iter: u16) -> (f32, f32) {
    let mut zn = z;
    let mut count = 0;

    while count < max_iter && zn.arg_sq() < max_divergence {
        zn = zn.pow(2) + c;
        count += 1;
    }
    (
        zn.arg_sq() as f32 / max_divergence as f32,
        count as f32 / max_iter as f32,
    )
}

///Compute mandelbrot fractal value for given parameters
/// * `pixel_complexe` - The complex number to compute the mandelbrot fractal value for c
/// * `max_iter` - The maximum number of iterations
/// * Return: a tuple of two f32 values (zn, count)
pub fn mandelbrot(pixel_complexe: Complex, max_iter: u16) -> (f32, f32) {
    let c = pixel_complexe;
    let mut zn = Complex::new(0 as f64, 0 as f64);
    let mut count = 0;

    while zn.arg_sq() < 4 as f64 && count < max_iter {
        zn = zn.pow(2) + c;
        count += 1;
    }
    (
        zn.arg_sq() as f32 / 4 as f32,
        count as f32 / max_iter as f32,
    )
}

///Compute iterated sin(z) fractal value for given parameters
/// * `z` - The complex number to compute the iterated sin(z) fractal value for z
/// * `c` - The complex number to compute the iterated sin(z) fractal value for c
/// * `max_iter` - The maximum number of iterations
/// * Return: a tuple of two f32 values (zn, count)
pub fn iterated_sin_z(z: Complex, c: Complex, max_iter: u16) -> (f32, f32) {
    let mut zn = z;
    let mut count = 0;

    while zn.arg_sq() < 50 as f64 && count < max_iter {
        zn = zn.sin() * c;
        count += 1;
    }
    (
        zn.arg_sq() as f32 / 4 as f32,
        count as f32 / max_iter as f32,
    )
}

///Compute newton raphson z^3 fractal value for given parameters
/// * `z` - The complex number to compute the newton raphson z^3 fractal value for z
/// * `max_iter` - The maximum number of iterations
/// * Return: a tuple of two f32 values (zn, count)
pub fn newton_raphson_z_3(z: Complex, max_iter: u16) -> (f32, f32) {
    let mut zn = z;
    let mut previous_zn = Complex::new(0.0, 0.0);
    let mut count = 0;

    while (zn - previous_zn).arg_sq() > 10.0_f64.powi(-6) && count < max_iter {
        previous_zn = zn;
        zn = zn - (zn.pow(3) - 1.0) / (zn.pow(2) * 3.0);
        count += 1;
    }

    (
        0.5 + zn.arg() as f32 / (2.0 * PI) as f32,
        count as f32 / max_iter as f32,
    )
}

///Compute newton raphson z^4 fractal value for given parameters
/// * `z` - The complex number to compute the newton raphson z^4 fractal value for z
/// * `max_iter` - The maximum number of iterations
/// * Return: a tuple of two f32 values (zn, count)
pub fn newton_raphson_z_4(z: Complex, max_iter: u16) -> (f32, f32) {
    let mut zn = z;
    let mut previous_zn = Complex::new(0.0, 0.0);
    let mut count = 0;

    while (zn - previous_zn).arg_sq() > 10.0_f64.powi(-6) && count < max_iter {
        previous_zn = zn;
        zn = zn - (zn.pow(4) - 1.0) / (zn.pow(3) * 4.0);
        count += 1;
    }

    (
        0.5 + zn.arg() as f32 / (2.0 * PI) as f32,
        count as f32 / max_iter as f32,
    )
}

///Compute nova newton raphson z^3 fractal value for given parameters
/// * `pixel_complexe` - The complex number to compute the nova newton raphson z^3 fractal value for c
/// * `max_iter` - The maximum number of iterations
/// * Return: a tuple of two f32 values (zn, count)
pub fn nova_newton_raphson_z_3(pixel_complexe: Complex, max_iter: u16) -> (f32, f32) {
    let mut zn = Complex::new(1.0, 0.0);
    let c = pixel_complexe;
    let mut previous_zn = Complex::new(0.0, 0.0);
    let mut count = 0;

    while (zn - previous_zn).arg_sq() > 10.0_f64.powi(-6) && count < max_iter {
        previous_zn = zn;
        zn = c + zn - (zn.pow(3) - 1.0) / (zn.pow(2) * 3.0);
        count += 1;
    }

    (0 as f32, count as f32 / max_iter as f32)
}

///Compute nova newton raphson z^4 fractal value for given parameters
/// * `pixel_complexe` - The complex number to compute the nova newton raphson z^4 fractal value for c
/// * `max_iter` - The maximum number of iterations
/// * Return: a tuple of two f32 values (zn, count)
pub fn nova_newton_raphson_z_4(pixel_complexe: Complex, max_iter: u16) -> (f32, f32) {
    let mut zn = Complex::new(1.0, 0.0);
    let c = pixel_complexe;
    let mut previous_zn = Complex::new(0.0, 0.0);
    let mut count = 0;

    while (zn - previous_zn).arg_sq() > 10.0_f64.powi(-6) && count < max_iter {
        previous_zn = zn;
        zn = c + zn - (zn.pow(4) - 1.0) / (zn.pow(3) * 4.0);
        count += 1;
    }

    (0 as f32, count as f32 / max_iter as f32)
}

///Compute the color for a given parameter t
/// * `t` - The parameter to compute the color for
/// * Return: a tuple of three u8 values (r, g, b)
pub fn color(t: f64) -> [u8; 3] {
    let a = (0.5, 0.5, 0.5);
    let b = (0.5, 0.5, 0.5);
    let c = (1.0, 1.0, 1.0);
    let d = (0.0, 0.10, 0.20);
    let r = b.0 * (6.28318 * (c.0 * t + d.0)).cos() + a.0;
    let g = b.1 * (6.28318 * (c.1 * t + d.1)).cos() + a.1;
    let b = b.2 * (6.28318 * (c.2 * t + d.2)).cos() + a.2;
    [(255.0 * r) as u8, (255.0 * g) as u8, (255.0 * b) as u8]
}

///Generate a random string of 10 characters
/// * Return: a random string of 10 characters
fn generate_random_string() -> String {
    let mut rng = thread_rng();

    let random_number: u64 = rng.gen_range(1..=999_999_9999);

    format!("{:010}", random_number)
}

///Create an image from a vector of PixelIntensity
/// * `task` - The FragmentTask containing the resolution and fractal name
/// * `pixel_intensity_vec` - The vector of PixelIntensity to create the image from
/// * `path` - The path to save the image to
/// * Return: a Result containing an empty tuple or an ImageError. The image is saved to the specified path.
pub fn create_image(
    task: &FragmentTask,
    pixel_intensity_vec: &Vec<PixelIntensity>,
    path: Option<&str>,
) -> Result<(), ImageError> {
    let image_width = task.resolution.nx as u32;
    let image_height = task.resolution.ny as u32;

    let mut image_buffer = image::ImageBuffer::new(image_width, image_height);

    let mut count = 0;
    for (_x, _y, pixel) in image_buffer.enumerate_pixels_mut() {
        let t = match task.fractal {
            FractalDescriptor::Julia(_) => pixel_intensity_vec[count].zn as f64,
            FractalDescriptor::Mandelbrot(_) => pixel_intensity_vec[count].zn as f64,
            _ => pixel_intensity_vec[count].count as f64,
        };

        *pixel = image::Rgb(color((2.0 * t + 0.5) % 1.0));
        count += 1;
    }

    let path = path.unwrap_or("./images/");
    let file_path = format!("{}{}_{}.png", path, task.fractal, generate_random_string());

    // Créez le répertoire s'il n'existe pas
    if let Some(parent_dir) = std::path::Path::new(&file_path).parent() {
        if !parent_dir.exists() {
            if let Err(err) = fs::create_dir_all(parent_dir) {
                eprintln!("Error creating directory: {}", err);
            }
        }
    }

    image_buffer.save(&file_path)?;

    Ok(())
}

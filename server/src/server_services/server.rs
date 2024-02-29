//! # Utility Functions
//!
//! The `utility` module provides a set of utility functions used in the context of a client-server communication system for fractal processing. These functions include generating unique IDs, reading messages from a client, and formatting data to a vector of `PixelIntensity` instances.
//!
//! ## Usage
//!
//! The utility functions can be used to perform various tasks related to communication and data manipulation in the fractal processing system. These functions abstract away common operations and provide convenient interfaces for working with IDs, messages, and pixel intensities.
//!
//! ## Examples
//!
//! Generating a unique ID:
//!
//! ```rust
//! use your_module_name::generate_unique_id;
//!
//! let unique_id = generate_unique_id();
//! ```
//!
//! Reading a message from a client:
//!
//! ```rust
//! use std::io;
//! use std::net::TcpStream;
//! use your_module_name::read_message_from_client;
//!
//! let mut stream = TcpStream::connect("127.0.0.1:8080").expect("Connection failed");
//! let (fragment, datas) = read_message_from_client(&mut stream).expect("Failed to read message from client");
//!
//! println!("Fragment: {:?}", fragment);
//! println!("Datas: {:?}", datas);
//! ```
//!
//! Formatting data to a vector of `PixelIntensity`:
//!
//! ```rust
//! use your_module_name::format_data_to_pixel_intensity_vector;
//! use shared_lib::complementary_types::pixelintensity::PixelIntensity;
//!
//! let datas = vec![/*... raw data ...*/];
//! let pixel_intensities = format_data_to_pixel_intensity_vector(&datas);
//!
//! for intensity in pixel_intensities {
//!     println!("Pixel Intensity: {:?}", intensity);
//! }
//! ```

use std::collections::HashMap;
use std::net::TcpStream;
use std::process::exit;
use std::{env, io};

use complex_math::Complex;
use shared_lib::complementary_types::pixelintensity::PixelIntensity;

use rand::RngCore;
use shared_lib::complementary_types::point::Point;
use shared_lib::complementary_types::range::Range;
use shared_lib::complementary_types::resolution::Resolution;
use shared_lib::complementary_types::u8data::U8Data;
use shared_lib::fractal_implementation::fractal_calcul::color;
use shared_lib::messages::message::{Fragment, FragmentTask};
use shared_lib::messages_methods::messages_methods::read_message;

/// Structure to store:
/// * params: FragmentTask needed to be computed for the full fractal
/// * tasks_state: HashMap of FragmentTask sent to client for computation with their unique id
/// * calcul_state: HashMap of PixelIntensity (data computed) with the unique id of the FragmentTask corresponding
#[derive(Debug, Clone)]
pub struct FractalCalculState {
    //16 squares
    pub params: Vec<FragmentTask>,
    pub tasks_state: HashMap<Vec<u8>, FragmentTask>,
    pub calcul_state: HashMap<Vec<u8>, Vec<PixelIntensity>>,
}

/// to generate a unique id as a vector of 16 bytes
/// * Return: `Vec<u8>` - a vector of 16 bytes representing a unique id
pub fn generate_unique_id() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut result = [0; 16];

    rng.fill_bytes(&mut result);

    Vec::from(result)
}

/// to read a message from a client
/// * `stream` - a mutable reference to a TcpStream
/// * Return: Result<(Fragment, `Vec<u8>`), io::Error> - a result containing a tuple of Fragment and a vector of bytes  or an io::Error
pub fn read_message_from_client(stream: &mut TcpStream) -> Result<(Fragment, Vec<u8>), io::Error> {
    let (fragment, datas) = match read_message(stream) {
        Ok((Fragment::FragmentRequest(request), datas)) => {
            println!("Client Thread: Request received");
            ((Fragment::FragmentRequest(request)), datas)
        }
        Ok((Fragment::FragmentResult(result), datas)) => {
            println!("Client Thread: Result received");
            ((Fragment::FragmentResult(result)), datas)
        }
        Ok(_) => {
            println!("Unknown request received");
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Unknown request received",
            ));
        }
        Err(err) => {
            eprintln!("Error will reading message: {}", err);
            return Err(err);
        }
    };
    Ok((fragment, datas))
}

/// to format data to a vector of PixelIntensity
/// * `datas` - a reference to a vector of bytes (u8)
/// * Return: `Vec<PixelIntensity>` - a vector of PixelIntensity instances
pub fn format_data_to_pixel_intensity_vector(datas: &Vec<u8>) -> Vec<PixelIntensity> {
    let mut pixel_intensities = Vec::new();

    for chunk in datas.chunks_exact(std::mem::size_of::<PixelIntensity>()) {
        // Assurez-vous que le chunk a la taille correcte
        assert_eq!(chunk.len(), std::mem::size_of::<PixelIntensity>());

        // Convertissez chaque groupe d'octets en f32
        let zn_bytes: [u8; 4] = [chunk[0], chunk[1], chunk[2], chunk[3]];
        let count_bytes: [u8; 4] = [chunk[4], chunk[5], chunk[6], chunk[7]];

        let zn = f32::from_be_bytes(zn_bytes);
        let count = f32::from_be_bytes(count_bytes);

        let pixel_intensity = PixelIntensity::new(zn, count);
        pixel_intensities.push(pixel_intensity);
    }
    pixel_intensities
}

///function to create the params for the julia fractal
/// * Return: `Vec<FragmentTask>` - a vector of FragmentTask for Julia fractal
pub fn create_params_for_julia() -> Vec<FragmentTask> {
    let mut params = Vec::new();

    let step_size_x = (1.2 - (-1.2)) / 4.0;
    let step_size_y = (1.2 - (-1.2)) / 4.0;
    let mut min_x = -1.2;
    let mut min_y = -1.2;
    let mut max_x = -0.6;
    let mut max_y = -0.6;

    for _i in 0..16 {
        params.push(FragmentTask {
            id: U8Data::new(0, 16),
            fractal: shared_lib::fractal_implementation::fractal::FractalDescriptor::Julia(
                shared_lib::fractal_types::julia_descriptor::JuliaDescriptor {
                    c: Complex {
                        re: 0.285,
                        im: 0.013,
                    },
                    divergence_threshold_square: 4.0,
                },
            ),
            max_iteration: 64,
            resolution: Resolution { nx: 300, ny: 300 },
            range: Range {
                min: Point { x: min_x, y: min_y },
                max: Point { x: max_x, y: max_y },
            },
        });

        min_x = max_x;
        if min_x < 1.2 {
            max_x = max_x + step_size_x;
        } else {
            min_x = -1.2;
            max_x = -0.6;
            min_y = max_y;
            max_y = max_y + step_size_y;
        }
    }
    println!("Params created");

    params
}

///function to create the params for the mandelbrot fractal
/// * Return: `Vec<FragmentTask>` - a vector of FragmentTask for Mandelbrot fractal
pub fn create_params_for_mandelbrot() -> Vec<FragmentTask> {
    let mut params = Vec::new();

    let step_size_x = (1.2 - (-1.2)) / 4.0;
    let step_size_y = (1.2 - (-1.2)) / 4.0;
    let mut min_x = -1.2;
    let mut min_y = -1.2;
    let mut max_x = -0.6;
    let mut max_y = -0.6;

    for _i in 0..16 {
        params.push(FragmentTask {
            id: U8Data::new(0, 16),
            fractal: shared_lib::fractal_implementation::fractal::FractalDescriptor::Mandelbrot(
                shared_lib::fractal_types::mandelbrot::Mandelbrot {},
            ),
            max_iteration: 64,
            resolution: Resolution { nx: 300, ny: 300 },
            range: Range {
                min: Point { x: min_x, y: min_y },
                max: Point { x: max_x, y: max_y },
            },
        });

        min_x = max_x;
        if min_x < 1.2 {
            max_x = max_x + step_size_x;
        } else {
            min_x = -1.2;
            max_x = -0.6;
            min_y = max_y;
            max_y = max_y + step_size_y;
        }
    }
    println!("Params created");

    params
}

///function to create the params for the iterated sin z fractal
/// * Return: `Vec<FragmentTask>` - a vector of FragmentTask for IteratedSinZ fractal
pub fn create_params_for_iterated_sin_z() -> Vec<FragmentTask> {
    let mut params = Vec::new();

    let step_size_x = (1.2 - (-1.2)) / 4.0;
    let step_size_y = (1.2 - (-1.2)) / 4.0;
    let mut min_x = -1.2;
    let mut min_y = -1.2;
    let mut max_x = -0.6;
    let mut max_y = -0.6;

    for _i in 0..16 {
        params.push(FragmentTask {
            id: U8Data::new(0, 16),
            fractal: shared_lib::fractal_implementation::fractal::FractalDescriptor::IteratedSinZ(
                shared_lib::fractal_types::iterated_sin_z::IteratedSinZ {
                    c: Complex { re: 1.0, im: 0.3 },
                },
            ),
            max_iteration: 64,
            resolution: Resolution { nx: 300, ny: 300 },
            range: Range {
                min: Point { x: min_x, y: min_y },
                max: Point { x: max_x, y: max_y },
            },
        });

        min_x = max_x;
        if min_x < 1.2 {
            max_x = max_x + step_size_x;
        } else {
            min_x = -1.2;
            max_x = -0.6;
            min_y = max_y;
            max_y = max_y + step_size_y;
        }
    }
    println!("Params created");

    params
}

///function to create the params for the newton raphson z 3 fractal
/// * Return: `Vec<FragmentTask>` - a vector of FragmentTask for NewtonRaphsonZ3 fractal
pub fn create_params_for_newton_raphson_z_3() -> Vec<FragmentTask> {
    let mut params = Vec::new();

    let step_size_x = (1.2 - (-1.2)) / 4.0;
    let step_size_y = (1.2 - (-1.2)) / 4.0;
    let mut min_x = -1.2;
    let mut min_y = -1.2;
    let mut max_x = -0.6;
    let mut max_y = -0.6;

    for _i in 0..16 {
        params.push(FragmentTask {
            id: U8Data::new(0, 16),
            fractal:
                shared_lib::fractal_implementation::fractal::FractalDescriptor::NewtonRaphsonZ3(
                    shared_lib::fractal_types::newton_raphson_z_3::NewtonRaphsonZ3 {},
                ),
            max_iteration: 64,
            resolution: Resolution { nx: 300, ny: 300 },
            range: Range {
                min: Point { x: min_x, y: min_y },
                max: Point { x: max_x, y: max_y },
            },
        });

        min_x = max_x;
        if min_x < 1.2 {
            max_x = max_x + step_size_x;
        } else {
            min_x = -1.2;
            max_x = -0.6;
            min_y = max_y;
            max_y = max_y + step_size_y;
        }
    }
    println!("Params created");

    params
}

///function to create the params for the newton raphson z 4 fractal
/// * Return: `Vec<FragmentTask>` - a vector of FragmentTask for NewtonRaphsonZ4 fractal
pub fn create_params_for_newton_raphson_z_4() -> Vec<FragmentTask> {
    let mut params = Vec::new();

    let step_size_x = (1.2 - (-1.2)) / 4.0;
    let step_size_y = (1.2 - (-1.2)) / 4.0;
    let mut min_x = -1.2;
    let mut min_y = -1.2;
    let mut max_x = -0.6;
    let mut max_y = -0.6;

    for _i in 0..16 {
        params.push(FragmentTask {
            id: U8Data::new(0, 16),
            fractal:
                shared_lib::fractal_implementation::fractal::FractalDescriptor::NewtonRaphsonZ4(
                    shared_lib::fractal_types::newton_raphson_z_4::NewtonRaphsonZ4 {},
                ),
            max_iteration: 64,
            resolution: Resolution { nx: 300, ny: 300 },
            range: Range {
                min: Point { x: min_x, y: min_y },
                max: Point { x: max_x, y: max_y },
            },
        });

        min_x = max_x;
        if min_x < 1.2 {
            max_x = max_x + step_size_x;
        } else {
            min_x = -1.2;
            max_x = -0.6;
            min_y = max_y;
            max_y = max_y + step_size_y;
        }
    }
    println!("Params created");

    params
}

///function to create the params for the nova newton raphson z 3 fractal
/// * Return: `Vec<FragmentTask>` - a vector of FragmentTask for NovaNewtonRaphsonZ3 fractal
pub fn create_params_for_nova_newton_raphson_z_3() -> Vec<FragmentTask> {
    let mut params = Vec::new();

    let step_size_x = (1.2 - (-1.2)) / 4.0;
    let step_size_y = (1.2 - (-1.2)) / 4.0;
    let mut min_x = -1.2;
    let mut min_y = -1.2;
    let mut max_x = -0.6;
    let mut max_y = -0.6;

    for _i in 0..16 {
        params.push(FragmentTask {
            id: U8Data::new(0, 16),
            fractal:
                shared_lib::fractal_implementation::fractal::FractalDescriptor::NovaNewtonRaphsonZ3(
                    shared_lib::fractal_types::nova_newton_raphson_z_3::NovaNewtonRaphsonZ3 {},
                ),
            max_iteration: 64,
            resolution: Resolution { nx: 300, ny: 300 },
            range: Range {
                min: Point { x: min_x, y: min_y },
                max: Point { x: max_x, y: max_y },
            },
        });

        min_x = max_x;
        if min_x < 1.2 {
            max_x = max_x + step_size_x;
        } else {
            min_x = -1.2;
            max_x = -0.6;
            min_y = max_y;
            max_y = max_y + step_size_y;
        }
    }
    println!("Params created");

    params
}

///function to create the params for the nova newton raphson z 4 fractal
/// * Return: `Vec<FragmentTask>` - a vector of FragmentTask for NovaNewtonRaphsonZ4 fractal
pub fn create_params_for_nova_newton_raphson_z_4() -> Vec<FragmentTask> {
    let mut params = Vec::new();

    let step_size_x = (1.2 - (-1.2)) / 4.0;
    let step_size_y = (1.2 - (-1.2)) / 4.0;
    let mut min_x = -1.2;
    let mut min_y = -1.2;
    let mut max_x = -0.6;
    let mut max_y = -0.6;

    for _i in 0..16 {
        params.push(FragmentTask {
            id: U8Data::new(0, 16),
            fractal:
                shared_lib::fractal_implementation::fractal::FractalDescriptor::NovaNewtonRaphsonZ4(
                    shared_lib::fractal_types::nova_newton_raphson_z_4::NovaNewtonRaphsonZ4 {},
                ),
            max_iteration: 64,
            resolution: Resolution { nx: 300, ny: 300 },
            range: Range {
                min: Point { x: min_x, y: min_y },
                max: Point { x: max_x, y: max_y },
            },
        });

        min_x = max_x;
        if min_x < 1.2 {
            max_x = max_x + step_size_x;
        } else {
            min_x = -1.2;
            max_x = -0.6;
            min_y = max_y;
            max_y = max_y + step_size_y;
        }
    }
    println!("Params created");

    params
}

///function to color the pixel of the image_buffer
/// * `task` - a reference to a FragmentTask to get the coordinates of the pixel to color
/// * `pixel_intensity_vec` - a reference to a vector of PixelIntensity to get the zn value or count value to color the pixel accordingly
/// * `image_buffer` - a mutable reference to the image buffer to be colored
pub fn put_color_in_image(
    task: &FragmentTask,
    pixel_intensity_vec: &Vec<PixelIntensity>,
    image_buffer: &mut image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
) {
    let mut x = ((task.range.min.x + 1.2) / 2.4 * 1200.0) as u32;
    let mut y = ((task.range.min.y + 1.2) / 2.4 * 1200.0) as u32;

    let y_end = y + 300;
    let x_end = x + 300;

    let mut count = 0;
    while y < y_end && count < pixel_intensity_vec.len() {
        while x < x_end && count < pixel_intensity_vec.len() {
            let color = color(pixel_intensity_vec[count].zn as f64);
            image_buffer.put_pixel(x, y, image::Rgb(color));
            x += 1;
            count += 1;
        }
        x = ((task.range.min.x + 1.2) / 2.4 * 1200.0) as u32;
        y += 1;
    }
}

///function to get the arguments passed to the server
/// * Return: `String` - the fractal name to be calculated
pub fn parse_args() -> String {
    let mut fractal_to_calcul = String::from("Julia");
    let fractal_available = vec![
        "Julia",
        "Mandelbrot",
        "IteratedSinZ",
        "NewtonRaphsonZ3",
        "NewtonRaphsonZ4",
        "NovaNewtonRaphsonZ3",
        "NovaNewtonRaphsonZ4",
    ];

    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            // Utiliser les valeurs par défaut de fractal "Julia"
        }
        2 => {
            // Changer pour "--help" quand possible de lancer en exécutable
            if args[1] == "--help" {
                println!("Usage : ./server <flag>");
                println!("Flag: --fractal <fractal_name>");
                println!("fractal_name: Julia, Mandelbrot, IteratedSinZ, NewtonRaphsonZ3, NewtonRaphsonZ4, NovaNewtonRaphsonZ3, NovaNewtonRaphson");

                // Terminer le programme
                exit(0);
            }
            // Récupérer les arguments valides
            println!("wrong flag, missing information, try --help for more information",);
            exit(0);
        }
        3 => {
            // Récupérer les arguments valides
            let flag = args[1].clone();
            fractal_to_calcul = args[2].clone();
            if flag != "--fractal" || flag != "-f" {
                println!("wrong flag, missing information, try --help for more information",);
                exit(0);
            }
            if !fractal_available.contains(&&*fractal_to_calcul) {
                println!("wrong fractal name, try --help for more information",);
                exit(0);
            }
        }
        _ => {
            // Nombres d'arguments incorrects
            eprintln!("Error : Invalid number of arguments !");
            eprintln!("wrong flag, missing information, try --help for more information",);
            exit(1);
        }
    }
    fractal_to_calcul
}

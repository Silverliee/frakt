use std::{
    collections::HashMap,
    env, fs,
    net::TcpListener,
    process::exit,
    sync::mpsc::{self, Sender},
};

mod server_services;
use complex_math::Complex;
use shared_lib::{
    complementary_types::{
        pixelintensity::PixelIntensity, point::Point, range::Range, resolution::Resolution,
        u8data::U8Data,
    },
    fractal_implementation::fractal_calcul::color,
    messages::message::{Fragment, FragmentTask},
    messages_methods::messages_methods::send_message_to_client,
};

use crate::server_services::server::{
    format_data_to_pixel_intensity_vector, generate_unique_id, read_message_from_client,
};

#[derive(Debug, Clone)]
pub struct FractalCalculState {
    //16 squares
    pub params: Vec<FragmentTask>,
    pub tasks_state: HashMap<Vec<u8>, FragmentTask>,
    pub calcul_state: HashMap<Vec<u8>, Vec<PixelIntensity>>,
}

fn create_params_for_julia() -> Vec<FragmentTask> {
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

fn create_params_for_mandelbrot() -> Vec<FragmentTask> {
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

fn create_params_for_iterated_sin_z() -> Vec<FragmentTask> {
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

fn create_params_for_newton_raphson_z_3() -> Vec<FragmentTask> {
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

fn create_params_for_newton_raphson_z_4() -> Vec<FragmentTask> {
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

fn create_params_for_nova_newton_raphson_z_3() -> Vec<FragmentTask> {
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

fn create_params_for_nova_newton_raphson_z_4() -> Vec<FragmentTask> {
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

fn put_color_in_image(
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

fn parse_args() -> String {
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

    // Récupérer le nom de l'exécutable
    let elements: Vec<&str> = args[0].split('/').collect();

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
            if flag != "--fractal" {
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

fn main() {
    let listener = match TcpListener::bind("127.0.0.1:8787") {
        Ok(listener) => listener,
        Err(err) => {
            eprintln!("Error binding to address: {}", err);
            exit(1);
        }
    };

    println!("Server listening on 127.0.0.1:8787");
    let fractal_to_calcul = parse_args();

    let (tx, rx) = mpsc::channel();

    println!("create server thread");
    std::thread::spawn(move || {
        println!("Server Thread: I am created");
        let image_width = 1200 as u32;
        let image_height = 1200 as u32;

        let mut image_buffer = image::ImageBuffer::new(image_width, image_height);

        let params = match fractal_to_calcul.as_str() {
            "Julia" => create_params_for_julia(),
            "Mandelbrot" => create_params_for_mandelbrot(),
            "IteratedSinZ" => create_params_for_iterated_sin_z(),
            "NewtonRaphsonZ3" => create_params_for_newton_raphson_z_3(),
            "NewtonRaphsonZ4" => create_params_for_newton_raphson_z_4(),
            "NovaNewtonRaphsonZ3" => create_params_for_nova_newton_raphson_z_3(),
            "NovaNewtonRaphsonZ4" => create_params_for_nova_newton_raphson_z_4(),
            _ => create_params_for_julia(),
        };

        let tasks_state = HashMap::new();
        let calcul_state = HashMap::new();
        let mut fractal_calcul_state = FractalCalculState {
            params,
            tasks_state,
            calcul_state,
        };

        for received in rx {
            let (tx, fragment, datas): (Sender<(Fragment, Vec<u8>)>, Fragment, Vec<u8>) = received;
            println!("Server Thread: Received fragment and datas from client thread");

            match fragment {
                Fragment::FragmentRequest(_) => {
                    //recuperer une tache et l envoyer avec le tx.send(task)
                    if fractal_calcul_state.params.len() != 0 {
                        let id = generate_unique_id();
                        let task = match fractal_calcul_state.params.pop() {
                            Some(task) => task,
                            None => {
                                println!("Server Thread: No more task");
                                return;
                            }
                        };
                        //enregistrer la tache dans le state avec son id
                        fractal_calcul_state.tasks_state.insert(id.clone(), task);
                        match tx.send((Fragment::FragmentTask(task), id.clone())) {
                            Ok(_) => println!("Server Thread: send fragment task to client thread"),
                            Err(_) => println!(
                                "Server Thread: Error sending fragment task to client thread"
                            ),
                        };
                    }
                }
                Fragment::FragmentResult(result) => {
                    //recuperer le resultat et creer l image en cherchant la tache grace a l'id
                    let id_size = result.id.count as usize;
                    let (id, data_to_be_transformed) = datas.split_at(id_size);
                    let _id = id.to_vec();
                    let data_to_be_transformed = data_to_be_transformed.to_vec();

                    let pixel_intensities =
                        format_data_to_pixel_intensity_vector(&data_to_be_transformed);
                    fractal_calcul_state
                        .calcul_state
                        .insert(_id.clone(), pixel_intensities.clone());
                    println!("Server Thread: processed result on server thread");
                    let task_calculated = match fractal_calcul_state.tasks_state.get(&_id) {
                        Some(task) => task,
                        None => {
                            println!("Server Thread: No task found");
                            return;
                        }
                    };

                    put_color_in_image(&task_calculated, &pixel_intensities, &mut image_buffer);

                    //recuperer une tache et l envoyer avec le tx.send(task)
                    if fractal_calcul_state.params.len() != 0 {
                        let task = match fractal_calcul_state.params.pop() {
                            Some(task) => task,
                            None => {
                                println!("Server Thread: No more task");
                                return;
                            }
                        };
                        let new_id = generate_unique_id();
                        let _ = tx.send((Fragment::FragmentTask(task), new_id.clone()));
                        println!("Server Thread: send fragment task to client thread");

                        //enregistrer la tache dans le state avec son id

                        fractal_calcul_state
                            .tasks_state
                            .insert(new_id.clone(), task);
                    }
                }
                _ => {
                    println!("Unknown request received");
                }
            }
            if fractal_calcul_state.calcul_state.len() == 16 {
                let file_path = format!("images/server/full{fractal_to_calcul}.png");
                println!("Server Thread: create Full Image, path: {}", file_path);

                // Créez le répertoire s'il n'existe pas
                if let Some(parent_dir) = std::path::Path::new(&file_path).parent() {
                    if !parent_dir.exists() {
                        if let Err(err) = fs::create_dir_all(parent_dir) {
                            eprintln!("Error creating directory: {}", err);
                        }
                    }
                }

                match image_buffer.save(&file_path) {
                    Ok(_) => {
                        println!("Server Thread: Image saved");
                    }
                    Err(err) => {
                        eprintln!("Error saving image: {}", err);
                    }
                };
                break;
            }
        }
    });

    // accepter les connexions des clients
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("New client connected");
                // traiter chaque client dans un thread séparé
                let tx = tx.clone();
                std::thread::spawn(move || {
                    let (fragment, data) = match read_message_from_client(&mut stream) {
                        Ok((fragment, data)) => (fragment, data),
                        Err(e) => {
                            println!("Error reading message from client: {}", e);
                            return;
                        }
                    };
                    let (tx_from_client, rx) = mpsc::channel::<(Fragment, Vec<u8>)>();
                    match tx.send((tx_from_client, fragment, data)) {
                        Ok(_) => println!("Client Thread: send fragment and data to server thread"),
                        Err(_) => println!(
                            "Client Thread: Error sending fragment and data to server thread"
                        ),
                    };

                    match rx.recv() {
                        Ok(received) => {
                            let (task, id) = received;
                            println!("Client Thread: received fragment and id to server thread");
                            match send_message_to_client(&mut stream, task, id) {
                                Ok(_) => println!("Client Thread: send task to client for calcul"),
                                Err(_) => println!("Client Thread: Error sending task to client"),
                            };
                        }
                        Err(_) => {
                            println!("Client Thread: No more task");
                            return;
                        }
                    };

                    println!("New client disconnected");
                });
            }
            Err(e) => {
                println!("Error accepting connection: {}", e);
            }
        }
    }
}

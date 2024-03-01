//! # Fractal Server
//!
//! The `fractal_server` module contains the main functionality for a server that performs fractal calculations and communicates with clients using TCP connections.
//!
//! The server supports different types of fractals, including Julia, Mandelbrot, IteratedSinZ, NewtonRaphsonZ3, NewtonRaphsonZ4, NovaNewtonRaphsonZ3, and NovaNewtonRaphsonZ4. Clients can request tasks, and the server distributes fractal calculation tasks to clients. Upon completion, clients send back results, and the server generates a full image once all tasks are completed.
//!
//! ## Usage
//!
//! To start the server, run the executable with optional flags. The available flags are:
//!
//! - `--help`: Displays usage information.
//! - `--fractal=<fractal_name>`: Specifies the type of fractal to calculate (default is Julia).
//! - `--host=<host>`: Specifies the host to bind the server to (default is localhost).
//! - `--port=<port>`: Specifies the port to bind the server to (default is 8787).
//!
//! Example:
//!
//! ```sh
//! ./server --fractal=Mandelbrot
//! ./server --host=127.0.0.1 --port=8787
//! ./server 127.0.0.1
//! ```
//!
//! ## Fractal Types
//!
//! The server supports the following fractal types:
//!
//! - Julia
//! - Mandelbrot
//! - IteratedSinZ
//! - NewtonRaphsonZ3
//! - NewtonRaphsonZ4
//! - NovaNewtonRaphsonZ3
//! - NovaNewtonRaphsonZ4
//!
//! ## Server Thread
//!
//! The server spawns a dedicated thread to handle fractal calculations and client interactions. It listens for incoming client connections and delegates tasks to client threads. Once all tasks are completed, the server generates a full image of the fractal.
//!
//! ## Client Thread
//!
//! Each client connection is processed in a separate thread. Clients can request tasks from the server, perform the calculations, and send back results. The client thread communicates with the server thread using message passing.
//!

use std::{
    collections::HashMap,
    fs,
    net::TcpListener,
    process::exit,
    sync::mpsc::{self, Sender},
    thread,
    time::Duration,
};

mod server_services;
use shared_lib::{
    messages::message::Fragment, messages_methods::messages_methods::send_message_to_client,
};

use crate::server_services::server::{
    create_params_for_iterated_sin_z, create_params_for_julia, create_params_for_mandelbrot,
    create_params_for_newton_raphson_z_3, create_params_for_newton_raphson_z_4,
    create_params_for_nova_newton_raphson_z_3, create_params_for_nova_newton_raphson_z_4,
    format_data_to_pixel_intensity_vector, generate_unique_id, parse_args, put_color_in_image,
    read_message_from_client, reset_state, FractalCalculState,
};

fn main() {
    let (host, port, mut fractal_to_calcul) = parse_args();
    let adress = format!("{}:{}", host, port);
    let listener = match TcpListener::bind(&adress) {
        Ok(listener) => listener,
        Err(err) => {
            eprintln!("Error binding to address: {}", err);
            exit(1);
        }
    };

    println!("Server listening on {}", adress);

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
                    //si pas de tache, le serveur en genere automatiquement au bout de 5sec
                    if fractal_calcul_state.params.len() == 0 {
                        println!("Server Thread: No more task, waiting 5sec before generating a new fractal");
                        thread::sleep(Duration::from_secs(5));
                        fractal_to_calcul = reset_state(&mut fractal_calcul_state);
                    }
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
                        Err(_) => {
                            println!("Server Thread: Error sending fragment task to client thread")
                        }
                    };
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

                    //on construit l image globale au fur et a mesure que les resultats sont recupérés
                    put_color_in_image(&task_calculated, &pixel_intensities, &mut image_buffer);

                    //Si l'image est complete, la sauvegarder et vider le state
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
                        //on reset le state
                        fractal_calcul_state.calcul_state.clear();
                        fractal_calcul_state.tasks_state.clear();
                    }

                    //recuperer une tache et l envoyer avec le tx.send(task)
                    //si pas de tache, le serveur en genere automatiquement au bout de 5sec
                    if fractal_calcul_state.params.len() == 0 {
                        println!("Server Thread: No more task, waiting 5sec before generating a new fractal");
                        thread::sleep(Duration::from_secs(5));
                        fractal_to_calcul = reset_state(&mut fractal_calcul_state);
                    }
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
                _ => {
                    println!("Unknown request received");
                }
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

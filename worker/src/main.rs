//! # Fractal Processing Client
//!
//! The Fractal Processing Client is a Rust program designed to connect to a server and perform calculations for specific fractal models. It forms part of a distributed computing system where a server assigns tasks to clients, and clients return results.
//!
//! ## Usage
//!
//! The client can be configured with command-line arguments to specify the server's host and port. By default, it connects to the localhost on port 8787. The client continuously communicates with the server, requesting tasks, performing computations, and sending back results.
//!
//! ## Command-Line Arguments
//!
//! - `./client`: Run the client with default settings.
//! - `./client <host>`: Specify the server's host, using the default port (8787).
//! - `./client <host> <port>`: Specify both the server's host and port.
//!
//! ## Features
//!
//! - Dynamically handles command-line arguments to configure the connection.
//! - Establishes a connection to the server and continuously communicates.
//! - Requests tasks, performs computations, and sends back results.
//! - Through Rayon, the computation are parallelized.
//!
//! ## How to Run
//!
//! To run the client, execute the compiled binary from the command line. Optionally, specify the server's host and port using command-line arguments. For example:
//!
//! ```shell
//! ./client                  # Run with default settings.
//! ./client example.com      # Connect to 'example.com' on the default port.
//! ./client example.com 9090 # Connect to 'example.com' on port 9090.
//! ```
//!
//! ## Dependencies
//!
//! This program relies on the 'rand' crate for generating random numbers.
//!
//! ```toml
//! [dependencies]
//! rand = "0.8.5"
//! ```

use core::time;
use std::process::exit;
use std::thread;

mod client_services;
use client_services::worker::ClientServices;
fn main() {
    let (host, port) = client_services::worker::ClientServices::parse_args();

    //Connexion
    let mut client = match ClientServices::new(&host, port) {
        Ok(client) => {
            println!("Client created and connected");
            client
        }
        Err(_) => {
            eprintln!("Erreur lors de la création et connexion du client");
            std::process::exit(1);
        }
    };

    //send request to server
    match client.send_request() {
        Ok(_) => {
            println!("Request sent");
        }
        Err(err) => {
            eprintln!("Erreur lors de l'envoi de la requête : {}", err);
            exit(1);
        }
    };

    loop {
        //get task from server
        match client.get_task_from_server() {
            Ok(response) => {
                let (task, datas) = response;
                println!("Task received, {:?}", task);
                //do work (and create image from client)
                let datas_updated = match client.do_work(&task, datas) {
                    Ok(datas) => datas,
                    Err(err) => {
                        eprintln!("Erreur lors de la réalisation de la tâche : {}", err);
                        exit(1);
                    }
                };

                //send result to server (new connection needed) -> loop because result sent will make server send a new task
                client = match ClientServices::new(&host, port) {
                    Ok(client) => {
                        println!("Client created and connected");
                        client
                    }
                    Err(_) => {
                        eprintln!("Erreur lors de la création et connexion du client");
                        std::process::exit(1);
                    }
                };
                match client.send_result(&task, &datas_updated) {
                    Ok(_) => {
                        println!("Result sent");
                    }
                    Err(err) => {
                        eprintln!("Erreur lors de l'envoi du résultat : {}", err);
                        exit(1);
                    }
                };
            }
            Err(_) => {
                eprintln!("No data to read currently, waiting 5sec before new attempt");
                thread::sleep(time::Duration::from_secs(5));
            }
        };
    }
}

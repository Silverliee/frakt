use core::time;
use std::process::exit;
use std::{env, thread};

mod client_services;
use client_services::worker::ClientServices;

fn parse_args() -> (String, u16) {
    let mut host = String::from("localhost");
    let mut port = 8787;

    let args: Vec<String> = env::args().collect();

    // Récupérer le nom de l'exécutable
    let elements: Vec<&str> = args[0].split('/').collect();

    match args.len() {
        1 => {
            // Utiliser les valeurs par défaut de host et port
        }
        2 => {
            // Changer pour "--help" quand possible de lancer en exécutable
            if args[1] == "--help" {
                println!("Usage : ./client <name> <port>");
                // Terminer le programme
                exit(0);
            } else {
                // Récupérer les arguments valides
                host = args[1].clone();
                println!(
                    "Pas de port spécifié, utilisation du port par défaut : {}",
                    port
                );
            }
        }
        3 => {
            // Récupérer les arguments valides
            host = args[1].clone();
            port = match args[2].clone().parse() {
                Ok(port) => port,
                Err(_) => 8787,
            };
        }
        _ => {
            // Nombres d'arguments incorrects
            eprintln!("Erreur : Nombre incorrect d'arguments !");
            eprintln!("Usage : ./client <name> <port>");
            // Terminer le programme avec un code d'erreur
            exit(1);
        }
    }
    (host, port)
}

fn main() {
    let (host, port) = parse_args();

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

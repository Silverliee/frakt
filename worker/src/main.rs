use std::env;
use std::process::exit;

mod client_services;
use client_services::worker::ClientServices;

fn parse_args() -> (String, u16) {
    let mut host = String::from("localhost");
    let mut port = 8787;

    let args: Vec<String> = env::args().collect();

    // Récupérer le nom de l'exécutable
    let elements: Vec<&str> = args[0].split('/').collect();
    let exec = elements.last().unwrap();

    match args.len() {
        1 => {
            // Utiliser les valeurs par défaut de host et port
        }
        2 => {
            // Changer pour "--help" quand possible de lancer en exécutable
            if args[1] == "--help" {
                println!("Usage : ./{} <name> <port>", exec);
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
            port = args[2].clone().parse().unwrap();
        }
        _ => {
            // Nombres d'arguments incorrects
            eprintln!("Erreur : Nombre incorrect d'arguments !");
            eprintln!("Usage : ./{} <name> <port>", exec);
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

    let (mut task, mut datas);

    loop {
        //get task from server
        (task, datas) = match client.get_task_from_server() {
            Ok((task, datas)) => (task, datas),
            Err(err) => {
                eprintln!("Erreur lors de la réception de la tâche : {}", err);
                exit(1);
            }
        };

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
}

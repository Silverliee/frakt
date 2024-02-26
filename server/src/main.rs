use std::net::TcpListener;

mod server_services;
use server_services::server::handle_client;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8787").expect("Failed to bind to address");

    println!("Server listening on 127.0.0.1:8787");

    // accepter les connexions des clients
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // traiter chaque client dans un thread séparé
                std::thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("Error accepting connection: {}", e);
            }
        }
    }
}

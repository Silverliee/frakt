//! # Fractal Processing Client Services
//!
//! The `ClientServices` module provides functionality for a client to connect to a server and perform tasks related to fractal processing. It includes methods for establishing a connection, sending requests, receiving tasks from the server, performing computations, creating images, and sending results back to the server.
//!
//! ## Dependencies
//!
//! The module relies on the following external crates:
//!
//! - `std::io`: Standard I/O library for handling input and output operations.
//! - `std::net::TcpStream`: Standard network library for establishing a TCP connection.
//! - `shared_lib`: A shared library containing fractal implementations, messages, and utility methods.
//!
//! ## Usage
//!
//! To use the `ClientServices` module, create an instance of the `ClientServices` struct by establishing a connection to the server. The client can then request tasks, perform computations, and send results back to the server.
//!
//! ## Example
//!
//! ```rust
//! use std::io;
//! use std::io::Write;
//! use std::net::TcpStream;
//!
//! use shared_lib::fractal_implementation::fractal::FractalDescriptor;
//! use shared_lib::messages::message::FragmentResult;
//! use shared_lib::messages::message::FragmentTask;
//! use shared_lib::messages::message::{Fragment, FragmentRequest};
//! use shared_lib::messages_methods::messages_methods::read_message;
//! use shared_lib::messages_methods::messages_methods::send_message;
//!
//! pub struct ClientServices {
//!     stream: TcpStream,
//! }
//!
//! // Rest of your code...
//!
//! ```
//!

use std::env;
use std::io;
use std::io::Write;
use std::net::TcpStream;
use std::process::exit;

use shared_lib::fractal_implementation::fractal::FractalDescriptor;
use shared_lib::fractal_implementation::fractal_calcul::create_image;
use shared_lib::messages::message::FragmentResult;
use shared_lib::messages::message::FragmentTask;
use shared_lib::messages::message::{Fragment, FragmentRequest};
use shared_lib::messages_methods::messages_methods::read_message;
use shared_lib::messages_methods::messages_methods::send_message;

pub struct ClientServices {
    stream: TcpStream,
}

impl ClientServices {
    pub fn connect_to(host: &str, port: &u16) -> Result<TcpStream, io::Error> {
        let server_addr = format!("{}:{}", host, port);
        TcpStream::connect(server_addr)
    }

    pub fn new(host: &str, port: u16) -> Result<ClientServices, io::Error> {
        let stream = ClientServices::connect_to(host, &port)?;

        Ok(ClientServices { stream })
    }

    pub fn get_task_from_server(&mut self) -> Result<(FragmentTask, Vec<u8>), io::Error> {
        let (task, datas) = match read_message(&mut self.stream) {
            Ok((Fragment::FragmentTask(task), datas)) => {
                println!("Task received");
                (task, datas)
            }
            Ok(_) => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Should return a FragmentTask",
                ));
            }
            Err(err) => return Err(err),
        };

        Ok((task, datas))
    }

    pub fn send_request(&mut self) -> Result<(), io::Error> {
        //Create a Fragment from FragmentRequest
        let fragment_request = FragmentRequest {
            worker_name: String::from("Group4-4AL1-Fractanstique"),
            maximal_work_load: 10,
        };
        let request = Fragment::FragmentRequest(fragment_request);
        let data = Vec::new();
        println!("Request created");

        //Send the FragmentRequest (data empty for a request)
        send_message(&mut self.stream, request, &data)
    }

    pub fn send_result(&mut self, task: &FragmentTask, datas: &Vec<u8>) -> Result<(), io::Error> {
        //Create a Fragment from FragmentResult
        let fragment_result = FragmentResult::create(&task);
        let _result = Fragment::FragmentResult(fragment_result);
        println!("Result created");

        send_message(&mut self.stream, _result, &datas)?;
        Ok(())
    }

    pub fn do_work(
        &mut self,
        task: &FragmentTask,
        mut datas: Vec<u8>,
    ) -> Result<Vec<u8>, io::Error> {
        // generate the datas for the fractal calculation from the task
        let pixels_calculated = FractalDescriptor::get_datas(&task);
        println!("Pixels calculated");

        // create the image from client (path can be changed to the desired path)
        match create_image(&task, &pixels_calculated, Some("./images/worker/")) {
            Ok(_) => {
                println!("Image created");
            }
            Err(err) => {
                eprintln!("Error while creating image : {}", err);
            }
        }

        // add the datas calculated to the buffer
        for pixel in pixels_calculated {
            match datas.write_all(&pixel.zn.to_be_bytes()) {
                Ok(_) => {}
                Err(err) => {
                    eprintln!("Error while writting zn : {}", err);
                }
            }
            match datas.write_all(&pixel.count.to_be_bytes()) {
                Ok(_) => {}
                Err(err) => {
                    eprintln!("Error while writting count : {}", err);
                }
            }
        }
        println!("Datas is now completed and ready to be sent");
        Ok(datas)
    }

    ///function to get the arguments passed to the program
    pub fn parse_args() -> (String, u16) {
        let args: Vec<String> = env::args().collect();

        let host_argument = args
            .iter()
            .find(|arg| arg.starts_with("--ip="))
            .map(|arg| arg.trim_start_matches("--ip="));

        let port_argument = args
            .iter()
            .find(|arg| arg.starts_with("--port="))
            .map(|arg| arg.trim_start_matches("--port="));

        let mut host = match host_argument {
            Some(host) => {
                println!("Host argument: {}", host);
                host.to_string()
            }
            None => "localhost".to_string(),
        };

        let mut port = match port_argument {
            Some(host) => {
                println!("Port argument: {}", host);
                match host.parse::<u16>() {
                    Ok(port) => port,
                    Err(_) => {
                        eprintln!("Error while parsing port argument");
                        exit(1);
                    }
                }
            }
            None => 8787,
        };

        if args.len() == 2 {
            if args[1] == "--help" {
                println!("Usage : ./worker 0.0.0.0");
                println!("Usage : ./worker <flag>");
                println!("Flag: --ip=<ip_adress>");
                println!("Flag: --port=<port>");
                // Terminer le programme
                exit(0);
            }
            if !args[1].starts_with("--") {
                host = args[1].clone();
            }
        } else if args.len() == 3 {
            if !args[1].starts_with("--") {
                host = args[1].clone();
            }
            if !args[2].starts_with("--") {
                port = match args[2].clone().parse::<u16>() {
                    Ok(port) => port,
                    Err(_) => {
                        eprintln!("Error while parsing port argument");
                        exit(1);
                    }
                };
            }
        }

        (host.to_string(), port)
    }
}

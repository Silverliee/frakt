use std::io;
use std::io::Write;
use std::net::TcpStream;

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
                eprintln!("Erreur lors de la création de l'image : {}", err);
            }
        }

        // add the datas calculated to the buffer
        for pixel in pixels_calculated {
            datas.write_all(&pixel.zn.to_be_bytes()).unwrap();
            datas.write_all(&pixel.count.to_be_bytes()).unwrap();
        }
        println!("Datas is now completed and ready to be sent");
        Ok(datas)
    }
}

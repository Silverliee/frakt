use std::process::exit;
use std::{env, io};
use std::io::{Read, Write};
use std::net::TcpStream;

use shared_lib::complementary_types::pixelintensity::PixelIntensity;
use shared_lib::fractal_implementation::fractal::FractalDescriptor;
use shared_lib::fractal_implementation::fractal_calcul::create_image;
use shared_lib::messages::fragment_request::FragmentRequest;
use shared_lib::messages::fragment_result::FragmentResult;
use shared_lib::messages::fragment_task::FragmentTask;

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

    pub fn request_task(&mut self, request: FragmentRequest) -> Result<(FragmentTask, Vec<u8>),io::Error> {
        // Sérialiser la demande
        let serialized = request.serialize()?;
        let json_bytes = serialized.as_bytes();
    
        // Envoyer la longueur du message deux fois car aucune data, donc la taille du message est la taille du message JSON
        self.write_u32(json_bytes.len() as u32)?;
        self.write_u32(json_bytes.len() as u32)?;
    
        // Envoyer le contenu JSON
        self.stream.write_all(json_bytes)?;
    
        // Lire la réponse de la tâche
        Ok(self.read_task_response()?)
    }
    
    fn write_u32(&mut self, value: u32) -> io::Result<()> {
        self.stream.write_all(&value.to_be_bytes())
    }
    
    pub fn read_task_response(&mut self) -> Result<(FragmentTask, Vec<u8>),io::Error> {
        // Lire la taille totale du message
        let total_message_size = self.read_u32()?;
        
        // Lire la taille du message JSON
        let json_message_size = self.read_u32()?;

        // Lire le message JSON
        let json_message = self.read_exact(json_message_size)?;
    
        // Lire les données supplémentaires
        let data_buffer = self.read_exact(total_message_size - json_message_size)?;
    
        let json_message_str = String::from_utf8_lossy(&json_message);
        let task = FragmentTask::deserialize(&json_message_str)?;
        Ok((task, data_buffer))

    }
    
    fn read_u32(&mut self) -> io::Result<usize> {
        let mut buffer = [0; 4];
        self.stream.read_exact(&mut buffer)?;
        Ok(u32::from_be_bytes(buffer) as usize)
    }
    
    fn read_exact(&mut self, size: usize) -> Result<Vec<u8>,io::Error> {
        let mut buffer = vec![0; size];
        self.stream.read_exact(&mut buffer)?;
        Ok(buffer)
    }
    
    pub fn send_result(&mut self, result: FragmentResult, datas: Vec<PixelIntensity>, id: Vec<u8>) -> Result<(), io::Error>{
        let serialized = result.serialize()?;
        let json_bytes = serialized.as_bytes();
        let msg_len: u32 = json_bytes.len() as u32;
    
        // Total message size = message size + count (Id size in bytes) + number of pixels * ( 4 bytes for zn (u32) + 4 bytes for count (u32)).
        let total_msg_len: u32 = msg_len + (result.pixels.offset + result.pixels.count * (4 + 4));
        println!("{:?} {:?}", &datas[0].zn.to_be_bytes(), &datas[0].count.to_be_bytes());
    
        // Send Total message size
        self.write_u32(total_msg_len)?;
    
        // Send Json message size
        self.write_u32(msg_len)?;

        // Send Json message (FragmentResult)
        self.stream.write(json_bytes)?;

        // Send Id
        self.stream.write(&id)?;

        // Send zn and count for each pixel
        for pixel in datas {
            self.stream.write(&pixel.zn.to_be_bytes())?;
            self.stream.write(&pixel.count.to_be_bytes())?;
        }
    
        Ok(())
    }
    
}

fn parse_args() -> (String,u16) {
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
                println!("Pas de port spécifié, utilisation du port par défaut : {}", port);
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
    (host,port)

}

fn main() {
    let (host,port) = parse_args();

    let mut client = if let Ok(client) = ClientServices::new(&host, port) {
        client
    } else {
        eprintln!("Erreur lors de la création du client");
        std::process::exit(1);
    };

    let request = FragmentRequest::new(String::from("worker"), 10);

    let (task, id);
    if let Ok(result) = client.request_task(request){
        (task, id) = result;
        print!("Task received");
    } else {
        eprintln!("Erreur lors de la réception de la tâche");
        exit(1);
    };
    // if let Ok(task_string) = task.serialize() {
    //     println!("{}", task_string);
    // } else {
    //     eprintln!("Erreur lors de la sérialisation du message FragmentTask");       
    // }

    let datas = FractalDescriptor::get_datas(&task);
    if let Err(err) = create_image(&task, &datas) {
        eprintln!("Erreur lors de la création de l'image : {}", err);
    }

    let _result = FragmentResult::create(&task);
    // if let Ok(result_string) = _result.serialize() {
    //     println!("{}", result_string);
    // } else {
    //     eprintln!("Erreur lors de la sérialisation du message FragmentResult");       
    // }

    //make loop here so when a FragmentResult is sent, the worker takes another task
    client = if let Ok(client) = ClientServices::new(&host, port) {
        client
    } else {
        eprintln!("Erreur lors de la création du client");
        std::process::exit(1);
    };    
    if let Err(err) = client.send_result(_result, datas, id) {
        eprintln!("Erreur lors de l'envoi du résultat : {}", err);
        exit(1);
    };

    loop {
        let (task, id);
        if let Ok(result) = client.read_task_response(){
            (task, id) = result;
            print!("Task received");
        } else {
            eprintln!("Erreur lors de la réception de la tâche");
            exit(1);
        };
        // if let Ok(task_string) = task.serialize() {
        //     println!("{}", task_string);
        // } else {
        //     eprintln!("Erreur lors de la sérialisation du message FragmentTask");       
        // }

        let _result = FragmentResult::create(&task);
        // if let Ok(result_string) = _result.serialize() {
        //     println!("{}", result_string);
        // } else {
        //     eprintln!("Erreur lors de la sérialisation du message FragmentResult");       
        // }

        let datas = FractalDescriptor::get_datas(&task);
        if let Err(err) = create_image(&task, &datas) {
            eprintln!("Erreur lors de la création de l'image : {}", err);
        }
        client = if let Ok(client) = ClientServices::new(&host, port) {
            client
        } else {
            eprintln!("Erreur lors de la création du client");
            std::process::exit(1);
        };    
        if let Err(err) = client.send_result(_result, datas, id) {
            eprintln!("Erreur lors de l'envoi du résultat : {}", err);
            exit(1);
        };

    }
}


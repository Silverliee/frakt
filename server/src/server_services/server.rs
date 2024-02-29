use std::io;
use std::net::TcpStream;

use shared_lib::complementary_types::pixelintensity::PixelIntensity;

use rand::RngCore;
use shared_lib::messages::message::Fragment;
use shared_lib::messages_methods::messages_methods::read_message;

pub fn generate_unique_id() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut result = [0; 16];

    rng.fill_bytes(&mut result);

    Vec::from(result)
}

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

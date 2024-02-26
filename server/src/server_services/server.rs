use std::net::TcpStream;

use shared_lib::complementary_types::pixelintensity::PixelIntensity;
use shared_lib::complementary_types::range::Range;
use shared_lib::complementary_types::resolution::Resolution;
use shared_lib::complementary_types::u8data::U8Data;
use shared_lib::fractal_implementation::fractal_calcul::create_image;
use shared_lib::messages::message::FragmentRequest;
use shared_lib::messages::message::FragmentTask;

use rand::RngCore;
use shared_lib::messages::message::Fragment;
use shared_lib::messages_methods::messages_methods::{read_message, send_message};

fn generate_unique_id() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut result = [0; 16];

    rng.fill_bytes(&mut result);

    Vec::from(result)
}

pub fn handle_client(mut stream: TcpStream) {
    let (fragment, datas) = match read_message(&mut stream) {
        Ok((Fragment::FragmentRequest(task), datas)) => {
            println!("Request received");
            ((Fragment::FragmentRequest(task)), datas)
        }
        Ok((Fragment::FragmentResult(result), datas)) => {
            println!("Result received");
            ((Fragment::FragmentResult(result)), datas)
        }
        Ok(_) => {
            println!("Unknown request received");
            return;
        }
        Err(err) => {
            eprintln!("Error will reading message: {}", err);
            return;
        }
    };

    match fragment {
        Fragment::FragmentRequest(request) => {
            if let Ok((_task, _id)) = send_fragment_task(request, &mut stream) {
            } else {
                println!("Error sending FragmentTask");
            };
        }
        Fragment::FragmentResult(result) => {
            let id_size = result.id.count as usize;
            let (id, data_to_be_transformed) = datas.split_at(id_size);
            let _id = id.to_vec();
            let data_to_be_transformed = data_to_be_transformed.to_vec();

            let pixel_intensities = format_data_to_pixel_intensity_vector(&data_to_be_transformed);

            let task = create_fragment_task();
            if let Err(err) = create_image(&task, &pixel_intensities, Some("./images/server/")) {
                eprintln!("Erreur lors de la création de l'image : {}", err);
            }
        }
        _ => {
            println!("Received unknown fragment");
        }
    }

    // do loop
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

pub fn create_fragment_task() -> FragmentTask {
    let id = U8Data::new(0, 16);
    let fractal = shared_lib::fractal_implementation::fractal::FractalDescriptor::Julia(
        shared_lib::fractal_types::julia_descriptor::JuliaDescriptor {
            c: complex_math::Complex {
                re: 0.285,
                im: 0.013,
            },
            divergence_threshold_square: 4.0,
        },
    );
    let max_iteration = 64;
    let resolution = Resolution { nx: 300, ny: 300 };
    let range = Range {
        min: shared_lib::complementary_types::point::Point { x: -1.2, y: -1.2 },
        max: shared_lib::complementary_types::point::Point { x: -0.6, y: -0.6 },
    };

    let fragment_task = FragmentTask {
        id,
        fractal,
        max_iteration,
        resolution,
        range,
    };

    fragment_task
}

pub fn send_fragment_task(
    _request: FragmentRequest,
    stream: &mut TcpStream,
) -> core::result::Result<(FragmentTask, Vec<u8>), std::io::Error> {
    //voir pour lier l id et la r
    let fragment_task = create_fragment_task();

    let fragment = Fragment::FragmentTask(fragment_task.clone());

    let id = generate_unique_id();

    send_message(stream, fragment, &id).unwrap();

    Ok((fragment_task, id))
}

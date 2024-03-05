use gnuplot::{Color, Figure};
use rand::prelude::*;
use std::{thread, time};
use vector3::Vector3;
use quaternion::Quaternion;
use std::io;
use std::error::Error;
use serialport::{DataBits, FlowControl, Parity, StopBits};

mod quaternion;

const SERIAL_PORT_NAME: &str = "/dev/ttyS0";
const BAUD_RATE: u32 = 9600;
const BUFFER_SIZE: usize = 1024;

fn main() -> io::Result<()> {
    // Box
    let vertices = [
        (0.0, 0.0, 0.0),
        (1.0, 0.0, 0.0),
        (1.0, 1.0, 0.0),
        (0.0, 1.0, 0.0),
        (0.0, 0.0, 1.0),
        (1.0, 0.0, 1.0),
        (1.0, 1.0, 1.0),
        (0.0, 1.0, 1.0),
    ];

    // Define the edges of the box
    let indices = [
        (0, 1),
        (1, 2),
        (2, 3),
        (3, 0),
        (4, 5),
        (5, 6),
        (6, 7),
        (7, 4),
        (0, 4),
        (1, 5),
        (2, 6),
        (3, 7),
    ];

    let mut fg = Figure::new();

    // Open the serial port
    let mut port = serialport::new(SERIAL_PORT_NAME, BAUD_RATE)
        .data_bits(DataBits::Eight)
        .flow_control(FlowControl::None)
        .parity(Parity::None)
        .stop_bits(StopBits::One)
        .open()?;

    // Create a buffer to store incoming data
    let mut buffer: Vec<u8> = vec![0; BUFFER_SIZE];

    let mut rotation_quaternion = Quaternion::new(1.0, 0.0, 0.0, 0.0);

    loop {
        // Read from the serial port
        match port.read(&mut buffer) {
            Ok(bytes_read) => {
                // Process the received data
                if bytes_read > 0 {
                    let received_data = &buffer[..bytes_read];
                    let data_str = String::from_utf8_lossy(received_data);
                    let parts: Vec<&str> = data_str.trim().split(',').collect();

                    if parts.len() == 4 {
                        if let (Ok(w), Ok(x), Ok(y), Ok(z)) = (
                            parts[0].parse::<f64>(),
                            parts[1].parse::<f64>(),
                            parts[2].parse::<f64>(),
                            parts[3].parse::<f64>()) {
                            rotation_quaternion = Quaternion::new(w, x, y, z);
                        } else {
                            eprintln!("Error parsing received data");
                        }
                    } else {
                        eprintln!("Received data has an invalid number of parts");
                    }

                } 
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {
                // Handle timeout errors
                continue;
            }
            Err(e) => {
                // Handle other errors
                eprintln!("Error reading from serial port: {}", e);
                break;
            }
        };

        // Clear the previous plot
        fg.clear_axes();
        let ax = fg.axes3d();

        // Rotate the vertices
        let mut rotated_vertices = vec![];
        for &(x, y, z) in vertices.iter() {
            let v = Vector3::new(x, y, z);
            let rotated_v = rotation_quaternion.rotate_vector(&v);
            rotated_vertices.push((rotated_v.x, rotated_v.y, rotated_v.z));
        }

        // Plot the rotated box
        for &(i,j) in indices.iter() {
            ax.lines(
                &[rotated_vertices[i].0, rotated_vertices[j].0],
                &[rotated_vertices[i].1, rotated_vertices[j].1],
                &[rotated_vertices[i].2, rotated_vertices[j].2],
                &[Color("blue")],
            );
        }

        // Show the plot
        fg.show_and_keep_running().unwrap();

        // Delay for a short time to control the rotation speed
        thread::sleep(time::Duration::from_millis(50));
    }
    Ok(())
}

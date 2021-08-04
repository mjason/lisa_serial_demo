use std::{time::Duration};

fn main() {
    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        println!("{}", p.port_name);
    }

    let port_name = "/dev/tty.usbserial-14240";
    let baud_rate = 115_200;

    let port = serialport::new(port_name, baud_rate)
        .timeout(Duration::from_millis(30))
        .open();

    // let mut serial_buf: Vec<u8> = vec![0; 32];
    // port.read(serial_buf.as_mut_slice()).expect("Found no data!");

    match port {
        Ok(mut port) => {
            println!("Receiving data on {} at {} baud:", &port_name, &baud_rate);
            loop {
                match port.read_clear_to_send() {
                    Ok(t) => {
                        if t {
                            println!("CTS {}", t)
                        }
                    },
                    Err(e) => eprintln!("{:?}", e),
                }
                // match port.read_data_set_ready() {
                //     Ok(t) => {
                //         if t {
                //             println!("DSR {}", t)
                //         }
                //     },
                //     Err(e) => eprintln!("{:?}", e),
                // }
                // match port.read_carrier_detect() {
                //     Ok(t) => {
                //         if t {
                //             println!("CD {}", t)
                //         }
                //     },
                //     Err(e) => eprintln!("{:?}", e),
                // }
            }
        }
        Err(e) => {
            eprintln!("Failed to open \"{}\". Error: {}", port_name, e);
            ::std::process::exit(1);
        }
    }
}

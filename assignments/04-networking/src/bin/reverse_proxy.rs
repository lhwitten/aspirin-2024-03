use aspirin_eats::db::AspirinEatsDb;
use std::env;
use std::io::prelude::*;
use std::io::stdin;
use std::net::TcpListener;
use std::net::TcpStream;

fn read_stream(mut a_stream: TcpStream, mut buf: [u8; 1024]) -> Option<String> {
    match a_stream.read(&mut buf) {
        Ok(num_bytes) => {
            if num_bytes == 0 {
                println!("Connection closed by the server.");
                return None;
            }
            // Print the received message (convert buffer to string)
            let response = String::from_utf8_lossy(&buf[..num_bytes]).to_string();

            return Some(response);
        }
        Err(e) => {
            eprintln!("Failed to read from stream: {}", e);
            return None;
        }
    }
}
fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 3 {
        eprintln!("Usage: {} <proxy-from> <proxy-to>", args[0]);
        std::process::exit(2);
    }

    let proxy_addr = &args[1];
    let origin_addr = &args[2];

    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to port 8080");
    let mut buf = [0; 1024];
    //server code
    loop {
        // accept connections and process them serially
        for stream in listener.incoming() {
            match stream {
                Ok(mut a_stream) => {
                    let incoming = match read_stream(
                        a_stream.try_clone().expect("Failed to clone stream"),
                        buf,
                    ) {
                        Some(incoming) => {
                            println!("incoming request is: {}", incoming);
                            incoming // Keep the incoming string
                        }
                        None => {
                            println!("No response incoming");
                            continue; // Skip to the next iteration if inside a loop
                        }
                    };

                    // send to server

                    //origin_addr something like "127.0.0.12701"
                    let mut origin_stream =
                        TcpStream::connect(origin_addr).expect("failed to bind");

                    if let Err(e) = origin_stream.write(incoming.as_bytes()) {
                        eprintln!("Failed to write to stream: {}", e);
                        continue;
                    }

                    //read server

                    let incoming = match read_stream(origin_stream, buf) {
                        Some(incoming) => {
                            println!("server response is: {}", incoming);
                            incoming // Keep the incoming string
                        }
                        None => {
                            println!("No response incoming");
                            continue; // Skip to the next iteration if inside a loop
                        }
                    };

                    //end of working stream
                    //echo response to user

                    if let Err(e) = a_stream.write(incoming.as_bytes()) {
                        eprintln!("Failed to write back to user: {}", e);
                        continue;
                    }
                }
                Err(e) => {
                    panic!("couldn't get a valid stream")
                }
            }
        }
    }
}

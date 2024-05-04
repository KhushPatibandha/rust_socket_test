use std::io::{ Read, Write };
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to address");
    println!("Server listening to 127.0.0.1:8080");

    loop {
        match listener.accept() {
            Ok((mut stream, addr)) => {
                println!("New connection: {}", addr);
                loop {
                    let mut buff = [0; 1024];
                    match stream.read(&mut buff) {
                        Ok(0) => {
                            println!("Client disconnected");
                            break;
                        },
                        Ok(_) => {
                            let request = String::from_utf8_lossy(&buff[..]);
                            println!("Received Request: {}", request);

                            let mut input = String::new();

                            print!("Enter a message: ");
                            std::io::stdout().flush().unwrap();
                            std::io::stdin().read_line(&mut input).unwrap();
                            
                            let response = input.as_bytes();
                            if let Err(e) = stream.write(response) {
                                eprintln!("Failed to write response: {}", e);
                                break;
                            }
                        },
                        Err(e) => {
                            eprintln!("Failed to read from client: {}", e);
                            break;
                        }
                    }
                }
            },
            Err(e) => {
                eprintln!("Failed to establish connection: {}", e);
            }
        }
    }
}
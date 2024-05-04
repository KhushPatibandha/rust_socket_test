use std::io::{ Read, Write };
use std::net::TcpStream;

const LOCAL: &str = "127.0.0.1:8080";
fn main() {
    let mut client = TcpStream::connect(LOCAL).expect("Error connecting to the server");

    loop {
        let mut buffer = [0; 1024];
        let mut input = String::new();

        print!("Enter a message: ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();

        if input.trim() != ":exit" {
            client.write(input.as_bytes()).unwrap();
            client.read(&mut buffer).unwrap();
            println!("Server response: {}", String::from_utf8_lossy(&buffer));
        }

        if input.trim() == ":exit" {
            println!("Closing the connection");
            break;
        }
    }
}
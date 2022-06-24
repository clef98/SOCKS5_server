use std::io::Write;
use std::io;
use std::net::{TcpStream, TcpListener};
use std::io::Read;

fn handle_connection(address: String, mut stream: TcpStream) -> io::Result<()>{
    let mut buffer = vec![0u8; 512];
    stream.read(&mut buffer[0..2]).expect("TODO: panic message");
    if buffer[0] != 0x05 {
        return Err(std::io::Error::new(std::io::ErrorKind::ConnectionAborted, "Only socks5 protocol is supported!"));
    }
    if buffer[1] != 0x01 {
        return Err(std::io::Error::new(std::io::ErrorKind::ConnectionAborted, "Only connect cmd is supported!"));
    }
    stream.write(&[0x05u8, 0x00]).expect("Error with writing to stream.");
    stream.read(&mut buffer[0..4]).expect("Error with reading stream.");
    //UDP ASSOCIATE AND BIND ARE NOT SUPPORTED.
    let mut port_type = buffer[3];
    let mut flag = true;
    let mut port = String::new();
    match port_type {
        0x01 => {
            stream.read_exact(&mut buffer[0..6]).expect("TODO: panic message");
            println!("Port has been selected.");
        }
        0x02 => {
            std::io::ErrorKind::ConnectionAborted;
        }
        0x03 => {
            stream.read_exact(&mut buffer[0..1]).expect("TODO: panic message");
        }
        0x04 => {
            stream.read_exact(&mut buffer[0..18]).expect("TODO: panic message");
        }

        _ => {
            println!("Failed connection");
            flag = false;
            std::process::exit(1);
        }
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    println!("Please type the address you would like to connect to: ");
    let args: Vec<String> = std::env::args().collect();
    let address:&str  = &args[1];
    let listener = TcpListener::bind(address).unwrap();
    for stream in listener.incoming() {
        let address_clone = address.to_string().clone();
        match stream {
            Ok(stream) => {
                std::thread::spawn(move || { handle_connection(address_clone, stream) });
            }
            Err(e) => {
                println!("Failed to receive messages: {}", e);
                std::process::exit(1);
            }
        }
    }
    Ok(())
}

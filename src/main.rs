use std::io::Write;
use std::io;
use std::net::{TcpStream, TcpListener};
use std::io::Read;


fn handle_connection(address: String, mut stream: TcpStream) -> io::Result<()>{

    //Size buffer (vector) to be filled with data from stream.
    let mut buffer = vec![0u8; 512];

    //First two pieces of data are read in from stream into buffer indices, expect working as unwrap except providing a identifiable location for the error.
    stream.read(&mut buffer[0..2]).expect("Error with reading stream.");

    //Version and method are checked from client request, as only SOCKS5 and CONNECT are accepted, with
    // their corresponding hexadecimals below. Errors are returned if not found and the connection is aborted.
    if buffer[0] != 0x05 {
        return Err(std::io::Error::new(std::io::ErrorKind::ConnectionAborted, "Only socks5 protocol is supported!"));
    }

    //UDP ASSOCIATE and BIND are not supported.
    if buffer[1] != 0x01 {
        return Err(std::io::Error::new(std::io::ErrorKind::ConnectionAborted, "Only connect cmd is supported!"));
    }

    //Writing back to the stream, where 0x05 confirms the protocal version of SOCKS5 and 0x00 is "succeeded" in the reply field.
    stream.write(&[0x05u8, 0x00]).expect("Error with writing to stream.");

    //Third indice of buffer is filled with ATYP, or the address type.
    stream.read(&mut buffer[0..4]).expect("Error with reading stream.");
    let port_type = buffer[3];


    //let mut flag = true;
    //let mut port = String::new();

    //Switch case where the portype is matched with the hexadeciamls of either IPv4, an error (0x02 is not supported by SOCKS5), a domainname, or IPV6 in that order.
    //If the port_type is not received, then a error message is printed and the program exits.
    match port_type {
        0x01 => {
            stream.read(&mut buffer[0..6]).expect("TODO: panic message");
            println!("Port has been selected.");
        }
        0x02 => {
            return Err(std::io::Error::new(std::io::ErrorKind::ConnectionAborted, "Please input a proper address type in client request!"));
        }
        0x03 => {
            stream.read(&mut buffer[0..1]).expect("TODO: panic message");
        }
        0x04 => {
            stream.read(&mut buffer[0..18]).expect("TODO: panic message");
        }

        _ => {
            println!("Failed connection");
            std::process::exit(1);
        }
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    println!("Please type the address you would like to connect to: ");

    //env::args() collects CLIs into a vector. Only address is inputed into args.
    let args: Vec<String> = std::env::args().collect();
    let address:&str  = &args[1];

    //TCPListener binds to the given address and prepares for incoming TCP connections, unwrap calls panic macro in case address is not valid.
    let listener = TcpListener::bind(address).unwrap();
    for stream in listener.incoming() {

        //Clone of address is made per each stream because life of address clones ends in the thread. For a stream, we enter a new thread that process handle_connection.
        let address_clone = address.to_string().clone();

        //Switch case for validation of TCPStream, where Ok creates a new thread (simultaneous process) to handle the newfound connection.
        match stream {
            Ok(stream) => {
                std::thread::spawn(move || { handle_connection(address_clone, stream) });
            }

            //If stream is not able to be used in Ok, e.g. is not TCPStream, an Err value will return containing information about the error, not matching the Ok(stream) pattern.
            Err(e) => {
                println!("Failed to receive messages: {}", e);
                std::process::exit(1);
            }
        }
    }

    //For std::io::Result<()>, might add improved error catching.
    Ok(())
}

use std::io::Write;
use std::io;
use std::net::{TcpStream, TcpListener};
use std::io::Read;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddrV4, SocketAddrV6};


fn handle_connection(address: String, mut stream: TcpStream) -> io::Result<()>{

    //Size buffer (vector) to be filled with data from stream. Not sure if u8 is correct size.
    let mut buffer:Vec<u8> = vec![0; 512];

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

            //IP address is 4 u8s here, port is 2 more u8s.
            stream.read(&mut buffer[0..6]).expect("Unexpected request size, consult SOCKS5 protocal and try again. ");

            //Extracts port number from buffer, 2 u8s.
            let mut port_dest: u8 = Default::default();
            port_dest = buffer[4..6].as_ref();

            //IPV4 requires u8 by 4, an empty vector is initializd using default and the IP address is sliced in from derefencing buffer for 4 u8s. The IPv4Addr is assigned.
            let mut address_vector: [u8; 4] = Default::default();
            address_vector.copy_from_slice(&buffer[0..4]);
            let IP_address = Ipv4Addr::from( address_vector);

            //Connect using SocketAddrV4, passes in ip address and port.
            let socket_v4 = SocketAddrV4::new(IP_address,port_dest.into());
            println!("Port has been selected.");

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


fn main() {
    println!("Hello, world!");


    /*
    Determine TCP or UDP
     */

    // pass address to our boys in the back
    let address = "127.0.0.1:8200";
    tcp_listener(address);
    println!("Shutting down server listener");
}

fn tcp_listener(address: &str) {

    // set up listener binded to port at address
    let listener = match TcpListener::bind(address) {
        Ok(listener) => listener,
        Err(e) => {
            println!("Could not bind to address specified: {:?}", e);
            return;
        }
    };
    println!("Server listening on {}", address);

    // listen for incoming connections
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let addr = match stream.peer_addr() {
                    Ok(addy) => addy,
                    Err(e) => {
                        println!("Could not peer address: {:?}", e);
                        return;
                    }
                };
                println!("New connection: {}", addr);
                thread::spawn(move || {
                    handle_connection(stream);
                });
            }
            Err(e) => {
                println!("Could not receive connection: {:?}", e);
            }
        }
    }
}
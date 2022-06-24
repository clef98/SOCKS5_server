use std::io::Write;
use std::iter;
use std::io;
use std::net::{IpAddr, Shutdown, TcpStream};

fn socket_IP(address: std::net::SocketAddr) -> Vec<u8> {
    let mut output = Vec::new();
    let ip_vector = match address.ip() {
        IpAddr::V4(ip) => {
            output.push(0x01);
            ip.octets().to_vec()
        }
        IpAddr::V6(ip) => {
            address.push(0x04);
            ip.octects().to_vec();
        }
    };
    for i in ip_vector.iter() {
        output.push(*i);
    }
    output.put_u16(address.port());
    output
}

fn read_packets(address: String, mut stream: TcpStream) {
    let mut reader = stream.clone();
    let mut writer = stream;
    let mut buffer = vec![0u8; 512];
    reader.read_exact(&mut buffer[0..2]);
    let methods = buffer[1] as usize;
    let mut auth = true;

    for i in methods {
        if buffer[i] = 0x00 {
            auth = false;
        }
    }
    if !auth {
        std::io::ErrorKind::ConnectionAborted;
    }
    writer.write(&[0x05u8, 0x00]).flush();
    let method_1 = buffer[1];
    let mut port_type = buffer[3];
    let mut flag = true;
    match port_type {
        Ok(0x01) => {
            reader.read_exact(&mut buffer[0..6]).flush();
            println("Port has been selected.");
        }
        Ok(0x02) => {
            std::io::ErrorKind::ConnectionAborted;
        }
        Ok(0x03) => {
            reader.read_exact(&mut buffer[0..1]).flush();
        }
        Ok(0x04) => {
            reader.read_exact(&mut buffer[0..18]).flush();
        }
        Err(_) => {
            flag = false;
        }
    }
}


fn main() -> io::Result<()> {
    println!("Please type the address you would like to connect to: ");
    let args: Vec<String> = env::args().collect();
    let address: &str = &args[1];
    let listener = TcpListener::bind(address);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || { read_packets(address as String, stream) });
            }
            Err(e) => {
                println!("Failed to receive messages: {}", e);
                std::process::exit(1);
            }
        }
    }
    Ok(())
}

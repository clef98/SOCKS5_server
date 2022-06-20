use std::iter;
use std::net::IpAddr;

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
        for i in ip_vector.iter(){
            output.push(*i);
        }
    output.put_u16(address.port());
    output
}

fn main() {
    let matches = App::new("SOCKS5 Proxy Server Beginning")
        .version(env!("CARGO_PKG_VERSION")).arg()
    let bind_addr = String::from(matches.value_of("bind").unwrap_or("127.0.0.1"));
    let bind_port = String::from(matches.value_of("port").unwrap_or("8080"));
}

use igd::{PortMappingProtocol, search_gateway};
use std::error::Error;
use std::net::{Ipv4Addr, SocketAddrV4};


pub fn forward_port_igd(addr: &String) -> Result<(), Box<dyn Error>> {
    let port_str = addr.split(':').last().ok_or("No port specified in addr")?;
    let port: u16 = port_str.parse()?;

    let local_ip_str = get_local_ip()?;
    let local_ip: Ipv4Addr = local_ip_str.parse()?;

    let local_socket = SocketAddrV4::new(local_ip, port);

    let gateway = search_gateway(Default::default())?;

    let lease_duration_seconds = 3600;
    gateway.add_port(
        PortMappingProtocol::TCP,
        port,
        local_socket,
        lease_duration_seconds,
        "Zync",
    )?;

    println!("Port {} forwarded via UPnP to LAN IP {}.", port, local_ip);
    Ok(())
}
pub fn get_local_ip() -> Result<String, Box<dyn Error>> {
    use std::net::UdpSocket;
    let s = UdpSocket::bind("0.0.0.0:0")?;
    s.connect("8.8.8.8:80")?;
    let local_addr = s.local_addr()?;
    Ok(local_addr.ip().to_string())
}

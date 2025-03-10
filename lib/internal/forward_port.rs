use igd::{PortMappingProtocol, search_gateway};
use std::error::Error;
use std::net::{Ipv4Addr, SocketAddrV4};

pub fn forward_port_igd(addr: &String) -> Result<(), Box<dyn Error>> {
    // 1) Extract the port from an address like "0.0.0.0:8080"
    let port_str = addr.split(':').last().ok_or("No port specified in addr")?;
    let port: u16 = port_str.parse()?;

    // 2) Grab your local LAN IP (e.g. "192.168.0.8")
    let local_ip_str = get_local_ip()?;
    let local_ip: Ipv4Addr = local_ip_str.parse()?;

    // Construct a SocketAddrV4 for the internal address
    let local_socket = SocketAddrV4::new(local_ip, port);

    // 3) Discover the router via UPnP IGD
    let gateway = search_gateway(Default::default())?;

    // 4) Forward that TCP port for 3600 seconds (1 hour)
    let lease_duration_seconds = 3600;
    gateway.add_port(
        PortMappingProtocol::TCP,
        port,         // external port
        local_socket, // internal SocketAddrV4
        lease_duration_seconds,
        "My Rust App", // label shown in your router's port map
    )?;

    println!("Port {} forwarded via UPnP to LAN IP {}.", port, local_ip);
    Ok(())
}
pub fn get_local_ip() -> Result<String, Box<dyn Error>> {
    use std::net::UdpSocket;
    let s = UdpSocket::bind("0.0.0.0:0")?;
    s.connect("8.8.8.8:80")?;
    let local_addr = s.local_addr()?;
    Ok(local_addr.ip().to_string()) // e.g. "192.168.0.8"
}

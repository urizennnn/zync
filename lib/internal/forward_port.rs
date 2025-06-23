use igd::{search_gateway, PortMappingProtocol};
use std::error::Error;
use std::net::{Ipv4Addr, SocketAddrV4};

/// Forwards a TCP port on the local network gateway using UPnP.
///
/// Parses the port from the provided address string, determines the local IPv4 address, and requests the gateway to forward the specified port to the local machine for 3600 seconds with the description "Zync".
///
/// # Parameters
/// - `addr`: Address string containing the port to forward (must include a colon and port, e.g., "0.0.0.0:8080").
///
/// # Returns
/// `Ok(())` if the port forwarding is successful; otherwise, returns an error if parsing, local IP retrieval, gateway discovery, or port mapping fails.
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

    log::info!("Port {} forwarded via UPnP to LAN IP {}.", port, local_ip);
    Ok(())
}
/// Determines the local IPv4 address used for outbound connections.
///
/// Attempts to discover the local IP address by creating a UDP socket and connecting to a public IP address. This reveals the IP address assigned to the local network interface used for external communication.
///
/// # Returns
/// The local IPv4 address as a string if successful; otherwise, returns an error.
pub fn get_local_ip() -> Result<String, Box<dyn Error>> {
    use std::net::UdpSocket;
    let s = UdpSocket::bind("0.0.0.0:0")?;
    s.connect("8.8.8.8:80")?;
    let local_addr = s.local_addr()?;
    Ok(local_addr.ip().to_string())
}

pub fn close_port_forwarding(port: u16) -> Result<(), Box<dyn Error>> {
    let gateway = search_gateway(Default::default())?;
    gateway.remove_port(PortMappingProtocol::TCP, port)?;
    Ok(())
}

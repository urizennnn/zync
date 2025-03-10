use igd::{PortMappingProtocol, search_gateway};
use std::error::Error;
use std::net::{Ipv4Addr, SocketAddrV4};

/// Example placeholder for your own local IP function.
///
/// This uses a UDP “connect” trick to discover your LAN IP address.

/// Example of retrieving your public IP from the outside
pub async fn get_public_ip() -> Result<String, reqwest::Error> {
    let response = reqwest::get("https://api.ipify.org?format=text").await?;
    let ip = response.text().await?;
    Ok(ip)
}

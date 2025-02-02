use std::net::UdpSocket;

pub fn get_local_ip() -> Option<String> {
    let socket = UdpSocket::bind("0.0.0.0:0").ok()?;
    socket.connect("8.8.8.8:80").ok()?; // Dummy connection
    socket.local_addr().ok().map(|addr| addr.ip().to_string())
}
pub async fn get_public_ip() -> Result<String, reqwest::Error> {
    let response = reqwest::get("https://api.ipify.org?format=text").await?;
    let ip = response.text().await?;
    Ok(ip)
}

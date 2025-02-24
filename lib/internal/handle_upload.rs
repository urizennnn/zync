use std::error::Error;
use tcp_server::http::put::put;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

pub async fn handle_incoming_upload(
    stream: &mut TcpStream,
    buffer: &mut [u8],
) -> Result<(), Box<dyn Error>> {
    let n = stream.read(buffer).await?;
    let command = String::from_utf8_lossy(&buffer[..n]).trim().to_string();
    if command.starts_with("UPLOAD") {
        put(stream, buffer).await?;
    }
    Ok(())
}

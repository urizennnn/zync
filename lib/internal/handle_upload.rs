use std::error::Error;
use tcp_client::methods::upload::upload;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

pub async fn handle_incoming_upload(
    stream: &mut TcpStream,
    buffer: &mut [u8],
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let n = stream.read(buffer).await?;
    let command = String::from_utf8_lossy(&buffer[..n]).trim().to_string();
    match upload(stream, &command, buffer).await {
        Ok(_) => println!("File uploaded successfully"),
        Err(e) => eprintln!("Error uploading file: {}", e),
    }
    Ok(())
}

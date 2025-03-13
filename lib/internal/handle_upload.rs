use std::error::Error;
use tcp_server::http::put::put;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

pub async fn handle_incoming_upload(
    stream: &mut TcpStream,
    buffer: &mut [u8],
) -> Result<(), Box<dyn Error + Send + Sync>> {
    // Read the command from the stream
    let n = stream.read(buffer).await?;
    if n == 0 {
        eprintln!("No data received; connection may be closed");
        return Err("No data received in handle_incoming_upload".into());
    }
    let command = String::from_utf8_lossy(&buffer[..n]).trim().to_string();
    log::debug!("Received command for PUT: {:?}", command);

    // Call the updated PUT handler.
    match put(stream, buffer, &command).await {
        Ok(_) => println!("File uploaded successfully"),
        Err(e) => eprintln!("Error uploading file: {}", e),
    }
    Ok(())
}

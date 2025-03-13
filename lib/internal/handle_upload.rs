use std::error::Error;
use tcp_server::http::put::put;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::TcpStream;

pub async fn handle_incoming_upload(
    stream: &mut TcpStream,
    buffer: &mut [u8],
) -> Result<(), Box<dyn Error + Send + Sync>> {
    // Wrap the stream in a BufReader to read a complete line.
    let mut reader = BufReader::new(stream);
    let mut command_line = String::new();
    let n = reader.read_line(&mut command_line).await?;
    if n == 0 {
        eprintln!("No data received; connection may be closed");
        return Err("No data received in handle_incoming_upload".into());
    }
    let command = command_line.trim().to_string();
    log::debug!("Received command for PUT: {:?}", command);

    // Get the underlying stream back to pass to put.
    // (Note: reader.get_mut() returns a mutable reference to the inner stream.)
    match put(reader.get_mut(), buffer, &command).await {
        Ok(_) => println!("File uploaded successfully"),
        Err(e) => eprintln!("Error uploading file: {}", e),
    }
    Ok(())
}

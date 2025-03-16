use std::error::Error;
use tcp_server::http::put::put;

pub async fn handle_incoming_upload(
    stream: &mut TcpStream,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match put(stream).await {
        Ok(_) => println!("File uploaded successfully"),
        Err(e) => eprintln!("Error uploading file: {}", e),
    }
    Ok(())
}
use tokio::net::TcpStream;

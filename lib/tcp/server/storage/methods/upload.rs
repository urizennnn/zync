use std::error::Error;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub async fn upload(
    stream: &mut TcpStream,
    path: &str,
    buffer: &mut [u8],
) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(path).await?;
    log::info!("File opened: {}", path);

    let file_size = file.metadata().await?.len();
    stream
        .write_all(format!("UPLOAD {} {}\n", path, file_size).as_bytes())
        .await?;
    stream.flush().await?;

    let mut total_sent = 0;
    loop {
        let bytes_read = file.read(buffer).await?;
        if bytes_read == 0 {
            break;
        }
        stream.write_all(&buffer[..bytes_read]).await?;
        stream.flush().await?;

        total_sent += bytes_read;
        log::info!("Progress: {}/{} bytes", total_sent, file_size);
    }

    log::info!("Upload complete: {} bytes sent", total_sent);
    Ok(())
}

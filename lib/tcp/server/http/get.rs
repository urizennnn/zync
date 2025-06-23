use std::{error::Error, path::Path};

use log::info;
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

pub async fn get_file(stream: &mut TcpStream, buffer: &mut [u8]) -> Result<(), Box<dyn Error>> {
    info!("Getting file..");
    let buf_string = String::from_utf8_lossy(buffer)
        .trim_matches(char::from(0))
        .trim()
        .to_string();
    let parts: Vec<&str> = buf_string.split_whitespace().collect();

    if parts.len() != 3 {
        stream.write_all(b"Invalid Command").await?;
        return Err("Invalid command".into());
    }
    let raw_path = parts[1];
    let format_path = format!("storage/{raw_path}");
    let path = Path::new(&format_path);
    let mut file = File::open(&path).await?;
    let file_size = file.metadata().await?.len();
    log::info!("Sending {format_path:?} ({} bytes)", file_size);
    stream
        .write_all(format!("SEND {} {}\n", format_path, file_size).as_bytes())
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
        info!("Progress: {}/{} bytes", total_sent, file_size);
    }

    info!("Upload complete: {} bytes sent", total_sent);

    info!("File sent");

    Ok(())
}

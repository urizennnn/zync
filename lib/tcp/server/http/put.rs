use crate::http::storage::STORAGE_PATH;
use log::{info, warn};
use std::error::Error;
use std::path::Path;
use tokio::{
    fs::{create_dir_all, File},
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

#[deny(clippy::never_loop)]
#[deny(clippy::ptr_arg)]
pub async fn put(stream: &mut TcpStream, buffer: &mut [u8]) -> Result<(), Box<dyn Error>> {
    println!("Processing PUT request");
    let buf = stream.read(buffer).await?;
    let buf_string = String::from_utf8_lossy(&buffer[..buf])
        .trim_matches(char::from(0))
        .trim()
        .to_string();
    let parts: Vec<&str> = buf_string.split_whitespace().collect();

    if parts.len() != 3 || parts[0] != "UPLOAD" {
        return Err("Invalid upload command".into());
    }

    let file_name = parts[1];
    let full_path = format!("{}{}", STORAGE_PATH, file_name);
    let file_size: u64 = parts[2].parse()?;

    info!("Uploading file: {} ({} bytes)", file_name, file_size);

    if let Some(parent) = Path::new(&full_path).parent() {
        create_dir_all(parent).await?;
    }

    let mut file = File::create(&full_path).await?;
    let mut remaining = file_size;

    while remaining > 0 {
        let to_read = std::cmp::min(buffer.len() as u64, remaining) as usize;
        let bytes_read = stream.read(&mut buffer[..to_read]).await?;
        if bytes_read == 0 {
            warn!("Unexpected end of file");
            return Err("Unexpected end of file".into());
        }
        file.write_all(&buffer[..bytes_read]).await?;
        remaining -= bytes_read as u64;
    }

    info!("File upload completed");
    stream.write_all(b"File uploaded successfully\n").await?;
    stream.flush().await?;
    Ok(())
}

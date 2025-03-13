use crate::http::storage::STORAGE_PATH;
use log::{info, warn};
use std::error::Error;
use std::path::Path;
use tokio::{
    fs::{File, create_dir_all},
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

#[deny(clippy::never_loop)]
#[deny(clippy::ptr_arg)]
pub async fn put(
    stream: &mut TcpStream,
    buffer: &mut [u8],
    command: &str,
) -> Result<(), Box<dyn Error>> {
    let parts: Vec<&str> = command.split_whitespace().collect();
    panic!("{:?}", parts);

    let file_name = parts[0];
    let full_path = format!("{}{}", STORAGE_PATH, file_name);

    if let Some(parent) = Path::new(&full_path).parent() {
        create_dir_all(parent).await?;
    }

    let mut file: File = File::create(&full_path).await?;
    let file_size = file.metadata().await?.len();
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

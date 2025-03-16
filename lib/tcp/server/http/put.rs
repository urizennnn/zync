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
pub async fn put(stream: &mut TcpStream, buffer: &mut [u8]) -> Result<(), Box<dyn Error>> {
    let n = stream.read(buffer).await?;
    panic!("{:?}", n);
    if n == 0 {
        return Err("No data received".into());
    }

    let input_str = String::from_utf8_lossy(&buffer[..n]).trim().to_string();

    let parts: Vec<&str> = input_str.split_whitespace().collect();
    if parts.len() < 2 {
        stream.write_all(b"Usage: PUT <filename>\n").await?;
        return Err("Invalid PUT command (missing filename)".into());
    }

    let file_name = parts[1];
    let full_path = format!("{:?}{:?}", STORAGE_PATH, file_name);

    if let Some(parent) = Path::new(&full_path).parent() {
        create_dir_all(parent).await?;
    }

    let mut file = File::create(&full_path).await?;

    let mut total_bytes_written = 0u64;
    loop {
        let n = stream.read(buffer).await?;
        if n == 0 {
            break; // done writing
        }
        file.write_all(&buffer[..n]).await?;
        total_bytes_written += n as u64;
    }

    info!("Uploaded {} bytes to {:?}", total_bytes_written, full_path);
    stream.write_all(b"File uploaded successfully\n").await?;
    stream.flush().await?;

    Ok(())
}

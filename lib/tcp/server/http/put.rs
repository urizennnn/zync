use crate::http::storage::STORAGE_PATH;
use log::info;
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
    if n == 0 {
        return Err("No data received".into());
    }
    panic!("{:?}", n);

    let input_str = String::from_utf8_lossy(&buffer[..n]).trim().to_string();
    let parts: Vec<&str> = input_str.split_whitespace().collect();
    if parts.len() < 2 {
        stream.write_all(b"Usage: PUT <filename>\n").await?;
        return Err("Invalid PUT command (missing filename)".into());
    }

    let file_name = parts[0];
    let file_size = parts[1].parse::<u64>()?;

    let full_path = format!("{:?}/{:?}", STORAGE_PATH, file_name);
    if let Some(parent) = Path::new(&full_path).parent() {
        create_dir_all(parent).await?;
    }

    let mut file = File::create(&full_path).await?;

    let mut total_bytes_written = 0u64;

    while total_bytes_written < file_size {
        let remaining = file_size - total_bytes_written;
        let to_read = std::cmp::min(remaining, buffer.len() as u64) as usize;

        let n = stream.read(&mut buffer[..to_read]).await?;
        if n == 0 {
            break;
        }

        file.write_all(&buffer[..n]).await?;
        total_bytes_written += n as u64;
        info!("Progress: {}/{} bytes", total_bytes_written, file_size);
    }

    info!("Uploaded {} bytes to {}", total_bytes_written, full_path);

    stream.write_all(b"File uploaded successfully\n").await?;
    stream.flush().await?;

    Ok(())
}

use std::path::PathBuf;
use tokio::{fs::File, io::AsyncReadExt, io::AsyncWriteExt, net::TcpStream};

#[deny(clippy::never_loop)]
#[deny(clippy::ptr_arg)]
pub async fn receive_files(
    stream: &mut TcpStream,
    buffer: &mut [u8],
    parts: &[&str],
    destination: &[&str],
) -> Result<(), Box<dyn std::error::Error>> {
    println!("{parts:?}");
    let source = destination[1];
    let destination = PathBuf::from(destination[2]);
    let final_path = destination.join(source);

    let file_size: u64 = parts[2].parse()?;
    log::info!("Receiving file: {} to {:?}", source, final_path);

    if let Some(parent) = final_path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }
    let mut file = File::create(&final_path).await?;
    let mut remaining = file_size;

    while remaining > 0 {
        let bytes_to_read = std::cmp::min(remaining as usize, buffer.len());
        let n = stream.read(&mut buffer[..bytes_to_read]).await?;
        file.write_all(&buffer[..n]).await?;
        remaining -= n as u64;
    }

    log::info!("File received and saved to: {:?}", final_path);
    Ok(())
}

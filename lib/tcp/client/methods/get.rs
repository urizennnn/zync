use std::path::PathBuf;
use tokio::{fs::File, io::AsyncReadExt, io::AsyncWriteExt, net::TcpStream};

#[deny(clippy::never_loop)]
#[deny(clippy::ptr_arg)]
/// Receives a file over a TCP stream and saves it to the specified destination path asynchronously.
///
/// The function reads the file size and destination information from the provided slices, ensures the destination directory exists, and writes the received file data to disk.
///
/// # Parameters
/// - `buffer`: Temporary buffer used for reading data from the stream.
/// - `parts`: Slice containing protocol-specific metadata, where `parts[2]` is expected to be the file size as a string.
/// - `destination`: Slice containing destination information, where `destination[1]` is the filename and `destination[2]` is the directory path.
///
/// # Returns
/// Returns `Ok(())` if the file is received and saved successfully, or an error if any step fails.
pub async fn receive_files(
    stream: &mut TcpStream,
    buffer: &mut [u8],
    parts: &[&str],
    destination: &[&str],
) -> Result<(), Box<dyn std::error::Error>> {
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

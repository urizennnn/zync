use std::error::Error;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

/// Asynchronously uploads a file's contents over a TCP stream in chunks.
///
/// Opens the specified file, sends an upload initiation message with the file path and size,
/// then transmits the file data in buffered chunks over the provided TCP stream. Progress is logged
/// after each chunk, and the function completes when the entire file has been sent.
///
/// # Parameters
/// - `stream`: The TCP stream to which the file will be uploaded.
/// - `path`: The path to the file to upload.
/// - `buffer`: A mutable byte buffer used for reading file chunks.
///
/// # Returns
/// Returns `Ok(())` if the upload completes successfully, or an error if any I/O operation fails.
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
        // Throttle progress logs to ~1% increments
        let mut last_progress_log = 0;
        let log_interval = file_size / 100; // Log every 1% progress

        loop {
            // ... existing code ...
            total_sent += bytes_read;
-        log::info!("Progress: {}/{} bytes", total_sent, file_size);
+        if total_sent - last_progress_log >= log_interval || total_sent == file_size {
+            log::info!(
+                "Progress: {}/{} bytes ({:.1}%)",
+                total_sent,
+                file_size,
+                (total_sent as f64 / file_size as f64) * 100.0
+            );
+            last_progress_log = total_sent;
+        }
        }
    }

    log::info!("Upload complete: {} bytes sent", total_sent);
    Ok(())
}

use bytes::{Buf, Bytes};
use std::error::Error;
use tokio::{
    fs::{File, create_dir_all},
    io::AsyncWriteExt,
    net::TcpStream,
};
use tokio_stream::StreamExt;
use tokio_util::codec::{FramedRead, LengthDelimitedCodec};

use crate::http::storage::STORAGE_PATH;

pub async fn put(stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    // Wrap the stream in a length-delimited framed reader.
    let mut framed = FramedRead::new(stream, LengthDelimitedCodec::new());

    // Wait for a frame; if none is received, return an error.
    let frame = framed.next().await.ok_or("No frame received")??;

    // Split the frame into a filename and file bytes.
    let (filename, filebytes) = split_payload(frame.into());

    if filename.is_empty() {
        return Err("Filename is empty".into());
    }

    // Create the full path by joining STORAGE_PATH with the filename.
    let full_path = STORAGE_PATH.join(&filename);

    // Ensure that the parent directory exists.
    if let Some(parent) = full_path.parent() {
        create_dir_all(parent).await?;
    } else {
        return Err("Could not determine the parent directory for the file".into());
    }

    // Create and write the file.
    let mut file = File::create(&full_path).await?;
    file.write_all(&filebytes).await?;
    println!(
        "Wrote {} bytes to '{}'",
        filebytes.len(),
        full_path.display()
    );

    Ok(())
}

/// Splits a payload of the form `[filename, 0x00, file_data]` into its components.
fn split_payload(mut frame: Bytes) -> (String, Bytes) {
    if let Some(pos) = frame.iter().position(|&b| b == 0) {
        let fname_bytes = frame.split_to(pos);
        // Skip the delimiter.
        frame.advance(1);
        let filename = String::from_utf8_lossy(&fname_bytes).to_string();
        (filename, frame)
    } else {
        let filename = String::from_utf8_lossy(&frame).to_string();
        (filename, Bytes::new())
    }
}

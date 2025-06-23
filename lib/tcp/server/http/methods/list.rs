use serde_json::json;
use std::{error::Error, fs};
use tokio::{io::AsyncWriteExt, net::TcpStream};

pub async fn list_storage(stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    stream.write_all(b"Listing storage items...\n").await?;

    let mut file_names = Vec::new(); // Create a vector to hold file names
    let entries = fs::read_dir("storage")?; // Retrieve the entries in the "storage" directory
    for entry in entries {
        let entry = entry?; // Unwrap each entry safely
        let file_name = entry.file_name(); // Get the file name
        let file_name_str = file_name.to_string_lossy().to_string(); // Convert the file name to a string
        file_names.push(file_name_str); // Add the file name to the vector
    }

    let json = json!(file_names); // Serialize the vector to a JSON array
    let json_str = json.to_string(); // Convert the JSON array to a string

    log::info!("{json_str}");
    stream.write_all(json_str.as_bytes()).await?; // Send the JSON string over the stream
    stream.write_all(b"\n").await?; // Add a newline to indicate the end of the message
    stream.flush().await?; // Flush the stream to ensure all data is sent

    Ok(())
}

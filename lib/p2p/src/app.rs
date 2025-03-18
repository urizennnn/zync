use std::path::PathBuf;

use anyhow::Result;
use iroh::{Endpoint, protocol::Router};
use iroh_blobs::{
    net_protocol::Blobs,
    rpc::client::blobs::WrapOption,
    store::{ExportFormat, ExportMode},
    ticket::BlobTicket,
    util::SetTagOption,
};

pub async fn main() -> Result<()> {
    let endpoint = Endpoint::builder().discovery_n0().bind().await?;
    let blobs = Blobs::memory().build(&endpoint);

    let router = Router::builder(endpoint)
        .accept(iroh_blobs::ALPN, blobs.clone())
        .spawn()
        .await?;

    let blobs_client = blobs.client();

    let args: Vec<String> = std::env::args().skip(1).collect();
    let arg_refs: Vec<&str> = args.iter().map(String::as_str).collect();

    match arg_refs.as_slice() {
        ["send", filename] => {
            let filename: PathBuf = filename.parse()?;
            let abs_path = std::path::absolute(&filename)?;

            println!("Hashing file.");

            // keep the file in place and link it, instead of copying it into the in-memory blobs database
            let in_place = true;
            let blob = blobs_client
                .add_from_path(abs_path, in_place, SetTagOption::Auto, WrapOption::NoWrap)
                .await?
                .finish()
                .await?;

            let node_id = router.endpoint().node_id();
            let ticket = BlobTicket::new(node_id.into(), blob.hash, blob.format)?;

            println!("File hashed. Fetch this file by running:");
            println!(
                "cargo run --example transfer -- receive {ticket} {}",
                filename.display()
            );
        }
        ["receive", ticket, filename] => {
            let filename: PathBuf = filename.parse()?;
            let abs_path = std::path::absolute(filename)?;
            let ticket: BlobTicket = ticket.parse()?;

            println!("Starting download.");

            blobs_client
                .download(ticket.hash(), ticket.node_addr().clone())
                .await?
                .finish()
                .await?;

            println!("Finished download.");
            println!("Copying to destination.");

            blobs_client
                .export(
                    ticket.hash(),
                    abs_path,
                    ExportFormat::Blob,
                    ExportMode::Copy,
                )
                .await?
                .finish()
                .await?;

            println!("Finished copying.");
        }
        _ => {
            println!("Couldn't parse command line arguments: {args:?}");
            println!("Usage:");
            println!("    # to send:");
            println!("    cargo run --example transfer -- send [FILE]");
            println!("    # this will print a ticket.");
            println!();
            println!("    # to receive:");
            println!("    cargo run --example transfer -- receive [TICKET] [FILE]");
        }
    }

    // Gracefully shut down the node
    println!("Shutting down.");
    router.shutdown().await?;

    Ok(())
}

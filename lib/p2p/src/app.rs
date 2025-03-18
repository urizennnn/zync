use iroh::{Endpoint, protocol::Router};
use iroh_blobs::net_protocol::Blobs;

pub async fn p2p_listener(file: &str) -> anyhow::Result<()> {
    let endpoint = Endpoint::builder().discovery_n0().bind().await.unwrap();

    let blobs = Blobs::memory().build(&endpoint);
    let router = Router::builder(endpoint)
        .accept(iroh_blobs::ALPN, blobs.clone())
        .spawn()
        .await?;

    let client = blobs.client();
    router.shutdown().await?;
    Ok(())
}

use iroh::{Endpoint, protocol::Router};
use iroh_blobs::net_protocol::Blobs;

async fn p2p_listener() {
    let endpoint = Endpoint::builder().discovery_n0().bind().await.unwrap();

    let blobs = Blobs::memory().build(&endpoint);
    let router = Router::builder(endpoint)
        .accept(iroh_blobs::ALPN, blobs.clone())
        .spawn()
        .await?;
    router.shutdown().await;
    Ok(())
}

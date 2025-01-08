use std::error::Error;

use zync::{init::init_app, utils::poll::poll_future};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    poll_future(Box::pin(init_app()))
}

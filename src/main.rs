use std::error::Error;

use zync::init::init_app;

fn main() -> Result<(), Box<dyn Error>> {
    let runtime = tokio::runtime::Runtime::new()?;
    runtime.block_on(init_app())
}

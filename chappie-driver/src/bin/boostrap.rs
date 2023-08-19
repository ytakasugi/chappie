use std::sync::Arc;

use chappie_driver::{module::Modules, startup::startup};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let modules = Modules::new().await;
    startup(Arc::new(modules)).await;

    Ok(())
}

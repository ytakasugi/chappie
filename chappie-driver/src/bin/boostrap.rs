use chappie_driver::{module::Modules, startup::startup};
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let modules = Modules::new().await;
    startup(Arc::new(modules)).await;

    Ok(())
}

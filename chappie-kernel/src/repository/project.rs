use async_trait::async_trait;

use crate::model::project::NewProject;

#[async_trait]
pub trait ProjectRepository {
    async fn create(&self, source: NewProject) -> anyhow::Result<()>;
}

use async_trait::async_trait;

use crate::model::user_project::NewUserProject;

#[async_trait]
pub trait UserProjectRepository {
    async fn create(&self, source: NewUserProject) -> anyhow::Result<()>;
}

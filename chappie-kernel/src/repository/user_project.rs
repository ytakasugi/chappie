use async_trait::async_trait;

use crate::model::{project::Project, user::User, user_project::NewUserProject, Id};

#[async_trait]
pub trait UserProjectRepository {
    async fn create(&self, source: NewUserProject) -> anyhow::Result<()>;
    async fn delete(&self, user_id: &Id<User>, project_id: &Id<Project>) -> anyhow::Result<()>;
}

use async_trait::async_trait;

use crate::model::user::NewUser;

#[async_trait]
pub trait UserRepository {
    async fn create(&self, source: NewUser) -> anyhow::Result<()>;
}
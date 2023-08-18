use std::sync::Arc;

use derive_new::new;

use chappie_adapter::modules::RepositoriesModuleExt;
use chappie_kernel::repository::user::UserRepository;

use crate::model::user::CreateUser;

#[derive(new)]
pub struct UserUseCase<R: RepositoriesModuleExt> {
    repositories: Arc<R>,
}

impl<R: RepositoriesModuleExt> UserUseCase<R> {
    pub async fn create(&self, source: CreateUser) -> anyhow::Result<()> {
        self.repositories
            .user_repository()
            .create(source.try_into()?)
            .await
    }
}

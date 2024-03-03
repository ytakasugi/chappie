use crate::model::user::CreateUser;
use chappie_adapter::modules::RepositoriesModuleExt;
use chappie_kernel::repository::user::UserRepository;
use derive_new::new;
use std::sync::Arc;

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

#[cfg(test)]
mod test {
    use super::UserUseCase;
    use crate::model::user::CreateUser;
    use chappie_adapter::{modules::RepositoriesModule, persistence::database::Db};
    use std::sync::Arc;

    #[tokio::test]
    async fn test_user_usecase_create() {
        let modules = RepositoriesModule::new(Db::new().await);
        let usecase = UserUseCase::new(Arc::new(modules));

        let source = CreateUser::new(
            "useCaseTest001".to_string(),
            "useCaseTest@email.com".to_string(),
            "useCaseTestP@ssword".to_string(),
            "9".to_string(),
        );

        usecase.create(source).await.unwrap();
    }
}

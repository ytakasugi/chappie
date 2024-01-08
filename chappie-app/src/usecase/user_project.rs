use std::sync::Arc;

use derive_new::new;

use chappie_adapter::modules::RepositoriesModuleExt;
use chappie_kernel::repository::user_project::UserProjectRepository;

use crate::model::user_project::CreateUserProject;

#[derive(new)]
pub struct UserProjectUseCase<R: RepositoriesModuleExt> {
    repositories: Arc<R>,
}

impl<R: RepositoriesModuleExt> UserProjectUseCase<R> {
    pub async fn create(&self, source: CreateUserProject) -> anyhow::Result<()> {
        self.repositories
            .user_project_repository()
            .create(source.try_into()?)
            .await
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use chappie_adapter::{modules::RepositoriesModule, persistence::database::Db};

    use crate::model::user_project::CreateUserProject;

    use super::UserProjectUseCase;

    #[tokio::test]
    async fn test_user_project_usecase_create() {
        let modules = RepositoriesModule::new(Db::new().await);
        let usecase = UserProjectUseCase::new(Arc::new(modules));

        let source = CreateUserProject::new(
            "01H9G2743M5CJAY0V2ZY5CQ4ZC".to_string(),
            "01H9G2743M5CJAY0V2ZY5CQ4ZC".to_string(),
            "9".to_string(),
        );

        usecase.create(source).await.unwrap();
    }
}

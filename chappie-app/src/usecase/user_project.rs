use crate::model::user_project::CreateUserProject;
use chappie_adapter::modules::RepositoriesModuleExt;
use chappie_kernel::repository::user_project::UserProjectRepository;
use derive_new::new;
use std::sync::Arc;

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

    pub async fn delete(&self, user_id: String, project_id: String) -> anyhow::Result<()> {
        self.repositories
            .user_project_repository()
            .delete(&user_id.try_into()?, &project_id.try_into()?)
            .await
    }
}

#[cfg(test)]
mod test {
    use super::UserProjectUseCase;
    use crate::model::user_project::CreateUserProject;
    use chappie_adapter::{modules::RepositoriesModule, persistence::database::Db};
    use std::sync::Arc;

    #[tokio::test]
    async fn test_user_project_usecase_create() {
        let modules = RepositoriesModule::new(Db::new().await);
        let usecase = UserProjectUseCase::new(Arc::new(modules));
        let user_id: String = "01H9G2743M5CJAY0V2ZY5CQ4ZC".to_string();
        let project_id: String = "01H9G2743M5CJAY0V2ZY5CQ4ZC".to_string();
        // 前回のデータを削除
        usecase
            .delete(user_id.clone(), project_id.clone())
            .await
            .unwrap();

        let source = CreateUserProject::new(user_id.clone(), project_id.clone(), "9".to_string());
        // ユーザープロジェクトを作成
        usecase.create(source).await.unwrap();
    }
}

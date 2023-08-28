use std::sync::Arc;

use derive_new::new;

use chappie_adapter::modules::RepositoriesModuleExt;
use chappie_kernel::repository::project::ProjectRepository;

use crate::model::project::CreateProject;

#[derive(new)]
pub struct ProjectUseCase<R: RepositoriesModuleExt> {
    repositories: Arc<R>,
}

impl<R: RepositoriesModuleExt> ProjectUseCase<R> {
    pub async fn create(&self, source: CreateProject) -> anyhow::Result<()> {
        self.repositories
            .project_repository()
            .create(source.try_into()?)
            .await
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use chappie_adapter::{modules::RepositoriesModule, persistence::database::Db};

    use crate::model::project::CreateProject;

    use super::ProjectUseCase;

    #[tokio::test]
    async fn create() {
        let modules = RepositoriesModule::new(Db::new().await);
        let usecase = ProjectUseCase::new(Arc::new(modules));

        let source = CreateProject::new(
            "UseCaseTestProject".to_string(),
            "Usecase Test project".to_string(),
            "01H8Y39CDFYSY86PMB5D4C8YNA".to_string(),
        );

        usecase.create(source).await.unwrap();
    }
}

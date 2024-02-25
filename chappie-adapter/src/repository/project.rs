use async_trait::async_trait;

use chappie_kernel::model::project::NewProject;
use chappie_kernel::model::project::Project;

use chappie_kernel::repository::project::ProjectRepository;

use super::DatabaseRepository;
use crate::model::project::ProjectTable;

#[async_trait]
impl ProjectRepository for DatabaseRepository<Project> {
    async fn create(&self, source: NewProject) -> anyhow::Result<()> {
        let project_table: ProjectTable = source.try_into()?;

        let query = sqlx::query_file_as!(
            ProjectTabl,
            "sql/createProject.sql",
            project_table.project_id,
            project_table.project_title,
            project_table.project_name,
            project_table.description,
            project_table.parent_project_id,
            project_table.created_at,
            project_table.updated_at,
            project_table.manager_id,
        );

        self.execute(query).await
    }
}

#[cfg(test)]
mod test {
    use chappie_kernel::model::project::NewProject;
    use chappie_kernel::model::Id;
    use chappie_kernel::repository::project::ProjectRepository;
    use ulid::Ulid;

    use super::DatabaseRepository;
    use crate::persistence::database::Db;

    #[tokio::test]
    async fn test_create() {
        let db = Db::new().await;
        let repository = DatabaseRepository::new(db);
        let project_id = Ulid::new();
        let user_id = Ulid::new();
        let parent_project_id = Ulid::new();

        repository
            .create(NewProject::new(
                Id::new(project_id),
                "TestCreateProject".to_string(),
                "test_create_project".to_string(),
                "Test project".to_string(),
                Some(Id::new(parent_project_id)),
                Id::new(user_id),
            ))
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_create_no_parent() {
        let db = Db::new().await;
        let repository = DatabaseRepository::new(db);
        let project_id = Ulid::new();
        let user_id = Ulid::new();

        repository
            .create(NewProject::new(
                Id::new(project_id),
                "TestCreateProjectNoParent".to_string(),
                "test_create_project_no_parent".to_string(),
                "Test project No Parent".to_string(),
                None,
                Id::new(user_id),
            ))
            .await
            .unwrap();
    }
}

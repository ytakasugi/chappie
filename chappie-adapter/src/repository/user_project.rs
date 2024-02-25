use async_trait::async_trait;
use crate::model::user_project::UserProjectTable;
use chappie_kernel::model::user_project::NewUserProject;
use chappie_kernel::model::user_project::UserProject;
use chappie_kernel::repository::user_project::UserProjectRepository;
use super::DatabaseRepository;

#[async_trait]
impl UserProjectRepository for DatabaseRepository<UserProject> {
    async fn create(&self, source: NewUserProject) -> anyhow::Result<()> {
        let user_project_table: UserProjectTable = source.try_into()?;
        // let pool = self.pool.0.clone();

        let query = sqlx::query_file_as!(
            UserProjectTable,
            "sql/createUserProject.sql",
            user_project_table.user_id,
            user_project_table.project_id,
            user_project_table.role,
            user_project_table.created_at,
            user_project_table.updated_at,
        );

        self.execute(query).await
    }
}

#[cfg(test)]
mod test {
    use chappie_kernel::model::user_project::NewUserProject;
    use chappie_kernel::model::Id;
    use chappie_kernel::repository::user_project::UserProjectRepository;
    use ulid::Ulid;

    use super::DatabaseRepository;
    use crate::persistence::database::Db;

    #[tokio::test]
    async fn test_create() {
        let db = Db::new().await;
        let repository = DatabaseRepository::new(db);
        let user_id = Ulid::new();
        let project_id = Ulid::new();

        repository
            .create(NewUserProject::new(
                Id::new(user_id),
                Id::new(project_id),
                "9".to_string(),
            ))
            .await
            .unwrap();
    }
}

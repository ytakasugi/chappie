use async_trait::async_trait;

use chappie_kernel::model::user_project::NewUserProject;
use chappie_kernel::model::user_project::UserProject;

use chappie_kernel::repository::user_project::UserProjectRepository;

use super::DatabaseRepository;
use crate::model::user_project::NewUserProjectTable;

#[async_trait]
impl UserProjectRepository for DatabaseRepository<UserProject> {
    async fn create(&self, source: NewUserProject) -> anyhow::Result<()> {
        let user_project_table: NewUserProjectTable = source.try_into()?;
        let pool = self.pool.0.clone();
        let mut tx = pool.begin().await.unwrap();

        let _ = sqlx::query_file_as!(
            NewUserProjectTable,
            "sql/createUserProject.sql",
            user_project_table.user_id,
            user_project_table.project_id,
            user_project_table.role,
        )
        .execute(&mut *tx)
        .await;

        tx.commit()
            .await
            .unwrap_or_else(|_| panic!("Commit failed"));

        Ok(())
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
    async fn create_ticket() {
        let db = Db::new().await;
        let repository = DatabaseRepository::new(db);
        let id = Ulid::new();

        repository
            .create(NewUserProject::new(Id::new(id), 1, "9".to_string()))
            .await
            .unwrap();
    }
}

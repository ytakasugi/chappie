use async_trait::async_trait;

use chappie_kernel::model::project::NewProject;
use chappie_kernel::model::project::Project;

use chappie_kernel::repository::project::ProjectRepository;

use super::DatabaseRepository;
use crate::model::project::NewProjectTable;

#[async_trait]
impl ProjectRepository for DatabaseRepository<Project> {
    async fn create(&self, source: NewProject) -> anyhow::Result<()> {
        let project_table: NewProjectTable = source.try_into()?;
        // コネクションプール
        let pool = self.pool.0.clone();
        // トランザクションを開始する
        let mut tx = pool.begin().await.unwrap();

        let _ = sqlx::query_file_as!(
            NewProjectTable,
            "sql/createProject.sql",
            project_table.project_name,
            project_table.description,
            project_table.manager_id,
        )
        .execute(&mut *tx)
        .await;

        tx.commit()
            .await
            .unwrap_or_else(|_| panic!("Commit failed"));

        Ok(())
    }
}

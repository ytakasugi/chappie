use chrono::{NaiveDateTime, Local};
use sqlx::FromRow;

use chappie_kernel::model::project::NewProject;

#[derive(FromRow)]
pub struct ProjectTable {
    pub project_id: i32,
    pub project_name: String,
    pub description: String,
    pub parent_project_id: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub manager_id: String,
}

/**
 * 新規プロジェクト作成用の構造体
 */
pub struct NewProjectTable {
    pub project_name: String,
    pub description: String,
    pub parent_project_id: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub manager_id: String,
}

impl TryFrom<NewProject> for NewProjectTable {
    type Error = anyhow::Error;

    fn try_from(project: NewProject) -> Result<Self, Self::Error> {
        Ok(NewProjectTable {
            project_name: project.project_name,
            description: project.description,
            parent_project_id: project.parent_project_id,
            created_at: Local::now().naive_local(),
            updated_at: None,
            manager_id: project.manager_id.value.to_string(),
        })
    }
}

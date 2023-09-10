use chrono::{Local, NaiveDateTime};
use sqlx::FromRow;

use chappie_kernel::model::user_project::NewUserProject;

#[derive(FromRow)]
pub struct UserProjectTable {
    pub user_id: String,
    pub project_id: String,
    pub role: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

impl TryFrom<NewUserProject> for UserProjectTable {
    type Error = anyhow::Error;

    fn try_from(user_project: NewUserProject) -> Result<Self, Self::Error> {
        Ok(UserProjectTable {
            user_id: user_project.user_id.value.to_string(),
            project_id: user_project.project_id.value.to_string(),
            role: user_project.role,
            created_at: Local::now().naive_local(),
            updated_at: None,
        })
    }
}

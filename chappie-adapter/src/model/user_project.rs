use chrono::NaiveDateTime;
use sqlx::FromRow;

use chappie_kernel::model::user_project::NewUserProject;

#[derive(FromRow)]
pub struct UserProjectTable {
    pub user_id: String,
    pub project_id: i32,
    pub role: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

pub struct NewUserProjectTable {
    pub user_id: String,
    pub project_id: i32,
    pub role: String,
}

impl TryFrom<NewUserProject> for NewUserProjectTable {
    type Error = anyhow::Error;

    fn try_from(user_project: NewUserProject) -> Result<Self, Self::Error> {
        Ok(NewUserProjectTable {
            user_id: user_project.user_id.value.to_string(),
            project_id: user_project.project_id,
            role: user_project.role,
        })
    }
}

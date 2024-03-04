use chrono::{Local, NaiveDateTime};
use sqlx::FromRow;

use chappie_kernel::model::user_project::{NewUserProject, UserProject};

#[derive(FromRow)]
pub struct UserProjectTable {
    pub user_id: String,
    pub project_id: String,
    pub role: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

impl TryFrom<UserProjectTable> for UserProject {
    type Error = anyhow::Error;

    fn try_from(user_project: UserProjectTable) -> Result<Self, Self::Error> {
        Ok(UserProject::new(
            user_project.user_id.try_into()?,
            user_project.project_id.try_into()?,
            user_project.role,
            user_project.created_at,
            user_project.updated_at,
        ))
    }
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

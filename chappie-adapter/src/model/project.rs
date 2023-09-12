use chrono::{Local, NaiveDateTime};
use sqlx::FromRow;

use chappie_kernel::model::project::NewProject;

use super::helper::convert_id_to_string;

#[derive(FromRow)]
pub struct ProjectTable {
    pub project_id: String,
    pub project_title: String,
    pub project_name: String,
    pub description: String,
    pub parent_project_id: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub manager_id: String,
}

impl TryFrom<NewProject> for ProjectTable {
    type Error = anyhow::Error;

    fn try_from(project: NewProject) -> Result<Self, Self::Error> {
        Ok(ProjectTable {
            project_id: project.project_id.value.to_string(),
            project_title: project.project_title,
            project_name: project.project_name,
            description: project.description,
            parent_project_id: convert_id_to_string(project.parent_project_id),
            created_at: Local::now().naive_local(),
            updated_at: None,
            manager_id: project.manager_id.value.to_string(),
        })
    }
}

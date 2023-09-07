use chrono::NaiveDateTime;
use derive_new::new;

use super::Id;

#[allow(clippy::too_many_arguments)]
#[derive(new, Debug, PartialEq, Eq)]
pub struct Project {
    pub project_id: i32,
    pub project_name: String,
    pub description: String,
    pub parent_project_id: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub manager_id: Id<Project>,
}

#[allow(clippy::too_many_arguments)]
#[derive(new, Debug)]
pub struct NewProject {
    pub project_name: String,
    pub description: String,
    pub parent_project_id: Option<i32>,
    pub manager_id: Id<Project>,
}

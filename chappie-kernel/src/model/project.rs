use chrono::NaiveDateTime;
use derive_new::new;

use super::{user::User, Id};

#[allow(clippy::too_many_arguments)]
#[derive(new, Clone, Debug, PartialEq, Eq)]
pub struct Project {
    pub project_id: Id<Project>,
    pub project_title: String,
    pub project_name: String,
    pub description: String,
    pub parent_project_id: Option<Id<Project>>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub manager_id: Id<User>,
}

#[derive(new, Debug)]
pub struct NewProject {
    pub project_id: Id<Project>,
    pub project_title: String,
    pub project_name: String,
    pub description: String,
    pub parent_project_id: Option<Id<Project>>,
    pub manager_id: Id<User>,
}

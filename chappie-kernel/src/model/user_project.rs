use chrono::NaiveDateTime;
use derive_new::new;

use super::Id;

#[derive(new, Debug, PartialEq, Eq)]
pub struct UserProject {
    pub user_id: Id<UserProject>,
    pub project_id: i32,
    pub role: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(new, Debug)]
pub struct NewUserProject {
    pub user_id: Id<UserProject>,
    pub project_id: i32,
    pub role: String,
}

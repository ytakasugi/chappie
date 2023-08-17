use chrono::{DateTime, Local};
use derive_new::new;

use super::Id;

#[allow(clippy::too_many_arguments)]
#[derive(new, Debug, PartialEq, Eq)]
pub struct User {
    pub user_id: Id<User>,
    pub user_name: String,
    pub email: String,
    pub password: String,
    pub role: String,
    pub created_at: DateTime<Local>,
    pub updated_at: Option<DateTime<Local>>,
    pub delete_flag: bool,
}

#[allow(clippy::too_many_arguments)]
#[derive(new, Debug)]
pub struct NewUser {
    pub user_id: Id<User>,
    pub user_name: String,
    pub email: String,
    pub password: String,
    pub role: String,
    pub created_at: DateTime<Local>,
    pub updated_at: Option<DateTime<Local>>,
    pub delete_flag: bool,
}
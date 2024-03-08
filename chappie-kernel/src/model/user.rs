use argon2::password_hash::SaltString;
use chrono::NaiveDateTime;
use derive_new::new;

use super::Id;

#[allow(clippy::too_many_arguments)]
#[derive(new, Clone, Debug, PartialEq, Eq)]
pub struct User {
    pub user_id: Id<User>,
    pub user_name: String,
    pub email: String,
    pub password: String,
    pub salt: SaltString,
    pub role: String,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub delete_flag: bool,
}

#[allow(clippy::too_many_arguments)]
#[derive(new, Debug)]
pub struct NewUser {
    pub user_id: Id<User>,
    pub user_name: String,
    pub email: String,
    pub password: String,
    pub salt: SaltString,
    pub role: String,
}

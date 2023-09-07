use argon2::password_hash::SaltString;
use chrono::{Local, NaiveDateTime};
use sqlx::FromRow;

use chappie_kernel::model::user::NewUser;

#[derive(FromRow)]
pub struct UserTable {
    pub user_id: String,
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

pub struct NewUserTable {
    pub user_id: String,
    pub user_name: String,
    pub email: String,
    pub password: String,
    pub salt: SaltString,
    pub role: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

impl TryFrom<NewUser> for NewUserTable {
    type Error = anyhow::Error;

    fn try_from(user: NewUser) -> Result<Self, Self::Error> {
        Ok(NewUserTable {
            user_id: user.user_id.value.to_string(),
            user_name: user.user_name,
            email: user.email,
            password: user.password,
            salt: user.salt,
            role: user.role,
            created_at: Local::now().naive_local(),
            updated_at: None,
        })
    }
}

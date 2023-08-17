use chrono::{Local, DateTime};
use sqlx::FromRow;

use chappie_kernel::model::user::NewUser;

#[derive(FromRow)]
pub struct UserTable {
    pub user_id: String,
    pub user_name: String,
    pub email: String,
    pub password: String,
    pub role: String,
    pub created_at: DateTime<Local>,
    pub updated_at: Option<DateTime<Local>>,
    pub delete_flag: bool,
}

impl TryFrom<NewUser> for UserTable {
    type Error = anyhow::Error;

    fn try_from(user: NewUser) -> Result<Self, Self::Error> {
        Ok(UserTable {
            user_id: user.user_id.value.to_string(),
            user_name: user.user_name,
            email: user.email,
            password: user.password,
            role: user.role,
            created_at: user.created_at,
            updated_at: user.updated_at,
            delete_flag: user.delete_flag,
        })
    }
}
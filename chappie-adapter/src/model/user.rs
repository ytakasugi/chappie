use chrono::{Local, NaiveDateTime};
use sqlx::FromRow;

use chappie_kernel::model::user::NewUser;

#[derive(FromRow)]
pub struct UserTable {
    pub user_id: String,
    pub user_name: String,
    pub email: String,
    pub password: String,
    pub role: String,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
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
            status: "1".to_string(),
            created_at: Local::now().naive_local(),
            updated_at: Some(Local::now().naive_local()),
            delete_flag: false,
        })
    }
}

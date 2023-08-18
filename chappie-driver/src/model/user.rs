use chappie_app::model::user::CreateUser;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct JsonCreateUser {
    pub user_name: String,
    pub email: String,
    pub password: String,
    pub role: String,
}

impl From<JsonCreateUser> for CreateUser {
    fn from(u: JsonCreateUser) -> Self {
        CreateUser {
            user_name: u.user_name,
            email: u.email,
            password: u.password,
            role: u.role,
        }
    }
}

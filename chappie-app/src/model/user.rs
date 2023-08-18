use chappie_adapter::helper;
use chappie_kernel::model::{user::NewUser, Id};
use derive_new::new;

#[derive(new)]
pub struct CreateUser {
    pub user_name: String,
    pub email: String,
    pub password: String,
    pub role: String,
}

impl TryFrom<CreateUser> for NewUser {
    type Error = anyhow::Error;

    fn try_from(c: CreateUser) -> Result<Self, Self::Error> {
        let user_id = Id::gen();
        let salt = helper::generate_salt();

        Ok(NewUser::new(
            user_id,
            c.user_name,
            c.email,
            c.password,
            salt,
            c.role,
        ))
    }
}

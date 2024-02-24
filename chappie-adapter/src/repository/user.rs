use async_trait::async_trait;

use chappie_kernel::model::user::NewUser;
use chappie_kernel::model::user::User;

use chappie_kernel::repository::user::UserRepository;

use super::DatabaseRepository;
use crate::model::user::UserTable;

use super::helper;
use crate::persistence::database::execute;

#[async_trait]
impl UserRepository for DatabaseRepository<User> {
    async fn create(&self, source: NewUser) -> anyhow::Result<()> {
        let user_table: UserTable = source.try_into()?;
        // パスワードハッシュ化
        let password_hash = helper::hash(&user_table.password, &user_table.salt)
            .map_err(|e| anyhow::anyhow!("Hashing error: {:?}", e))?;
        // コネクションプール
        let pool = self.pool.0.clone();

        let query = sqlx::query_file_as!(
            UserTable,
            "sql/createUser.sql",
            user_table.user_id,
            user_table.user_name,
            user_table.email,
            password_hash,
            user_table.salt.to_string(),
            user_table.role,
            user_table.status,
            user_table.created_at,
            user_table.updated_at,
            user_table.delete_flag,
        );

        execute(pool, query).await
    }
}

#[cfg(test)]
mod test {
    use chappie_kernel::model::user::NewUser;
    use chappie_kernel::model::Id;
    use chappie_kernel::repository::user::UserRepository;
    use ulid::Ulid;

    use super::helper;
    use super::DatabaseRepository;
    use crate::persistence::database::Db;

    #[tokio::test]
    async fn test_create() {
        let db = Db::new().await;
        let repository = DatabaseRepository::new(db);
        let id = Ulid::new();
        let salt = helper::generate_salt();

        repository
            .create(NewUser::new(
                Id::new(id),
                "test_create_user".to_string(),
                "testCreateUser@email.com".to_string(),
                "P@ssword".to_string(),
                salt,
                "9".to_string(),
            ))
            .await
            .unwrap();
    }
}

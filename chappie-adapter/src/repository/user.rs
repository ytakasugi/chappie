use async_trait::async_trait;

use chappie_kernel::model::user::NewUser;
use chappie_kernel::model::user::User;

use chappie_kernel::repository::user::UserRepository;

use super::DatabaseRepository;
use crate::helper;
use crate::model::user::UserTable;

#[async_trait]
impl UserRepository for DatabaseRepository<User> {
    async fn create(&self, source: NewUser) -> anyhow::Result<()> {
        let user_table: UserTable = source.try_into()?;
        // パスワードハッシュ化
        let password_hash = helper::hash(&user_table.password, &user_table.salt).unwrap();
        // コネクションプール
        let pool = self.pool.0.clone();
        // トランザクションを開始する
        let mut tx = pool.begin().await.unwrap();

        let _ = sqlx::query_file_as!(
            UserTable,
            "sql/createUser.sql",
            user_table.user_id,
            user_table.user_name,
            user_table.email,
            password_hash,
            user_table.salt.to_string(),
            user_table.role,
        )
        .execute(&mut *tx)
        .await?;

        // トランザクションをコミットする
        tx.commit()
            .await
            .unwrap_or_else(|_| panic!("Commit failed."));

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use chappie_kernel::model::user::NewUser;
    use chappie_kernel::model::Id;
    use chappie_kernel::repository::user::UserRepository;
    use ulid::Ulid;

    use super::DatabaseRepository;
    use crate::helper;
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

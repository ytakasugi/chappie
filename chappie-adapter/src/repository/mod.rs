use std::marker::PhantomData;

use derive_new::new;

use crate::persistence::database::Db;

pub mod helper;
pub mod project;
pub mod ticket;
pub mod user;
pub mod user_project;

use sqlx::query::Query;

#[derive(new)]
pub struct DatabaseRepository<T> {
    pool: Db,
    _marker: PhantomData<T>,
}

impl<T> DatabaseRepository<T> {
    pub async fn execute<'a>(
        &self,
        query: Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> Result<(), anyhow::Error> {
        let mut tx = self.pool.0.begin().await?;
        match query.execute(&mut *tx).await {
            Ok(_) => {
                tx.commit().await?;
                Ok(())
            }
            Err(e) => {
                tx.rollback().await?;
                Err(anyhow::Error::new(e))
            }
        }
    }

    pub async fn execute_multiple<'a>(
        &self,
        queries: Vec<Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>>,
    ) -> Result<(), anyhow::Error> {
        let mut tx = self.pool.0.begin().await?;
        for query in queries {
            match query.execute(&mut *tx).await {
                Ok(_) => {}
                Err(e) => {
                    tx.rollback().await?;
                    return Err(anyhow::Error::new(e));
                }
            }
        }
        tx.commit().await?;
        Ok(())
    }
}

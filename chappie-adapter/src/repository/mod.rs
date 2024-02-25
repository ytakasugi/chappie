use crate::persistence::database::Db;
use derive_new::new;
use sqlx::{query::Query, Pool, Postgres};
use std::{marker::PhantomData, sync::Arc};

pub mod helper;
pub mod project;
pub mod ticket;
pub mod user;
pub mod user_project;

#[derive(new)]
pub struct DatabaseRepository<T> {
    pool: Db,
    _marker: PhantomData<T>,
}

impl<T> DatabaseRepository<T> {
    pub fn cloned_connection_pool(&self) -> Arc<Pool<Postgres>> {
        self.pool.0.clone()
    }

    pub async fn execute<'a>(
        &self,
        query: Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> Result<(), anyhow::Error> {
        let mut tx = self.cloned_connection_pool().begin().await?;
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
        let mut tx = self.cloned_connection_pool().begin().await?;

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

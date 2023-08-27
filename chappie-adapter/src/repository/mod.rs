use std::marker::PhantomData;

use derive_new::new;

use crate::persistence::database::Db;

pub mod project;
pub mod user;

#[derive(new)]
pub struct DatabaseRepository<T> {
    pool: Db,
    _marker: PhantomData<T>,
}

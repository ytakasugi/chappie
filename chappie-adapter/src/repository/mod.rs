use std::marker::PhantomData;

use derive_new::new;

use crate::persistence::database::Db;

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

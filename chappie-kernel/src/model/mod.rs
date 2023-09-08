use anyhow::anyhow;
use derive_new::new;
use std::marker::PhantomData;
use ulid::Ulid;

pub mod project;
pub mod ticket;
pub mod user;
pub mod user_project;

#[derive(new, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Id<T> {
    pub value: Ulid,
    _marker: PhantomData<T>,
}

impl<T> Id<T> {
    pub fn gen() -> Id<T> {
        Id::new(Ulid::new())
    }
}

impl<T> TryFrom<String> for Id<T> {
    type Error = anyhow::Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ulid::from_string(&value)
            .map(|id| Self::new(id))
            .map_err(|err| anyhow!("{:?}", err))
    }
}
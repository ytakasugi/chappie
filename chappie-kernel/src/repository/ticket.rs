use async_trait::async_trait;

use crate::model::ticket::{NewTicket, Ticket};

#[async_trait]
pub trait TicketRepository {
    async fn create(&self, source: NewTicket) -> anyhow::Result<()>;
    async fn find(&self, id: i32) -> anyhow::Result<Option<Ticket>>;
}

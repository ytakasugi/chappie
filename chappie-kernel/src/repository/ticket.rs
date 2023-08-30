use async_trait::async_trait;

use crate::model::ticket::NewTicket;

#[async_trait]
pub trait TicketRepository {
    async fn create(&self, source: NewTicket) -> anyhow::Result<()>;
}

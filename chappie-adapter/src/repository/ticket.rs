use async_trait::async_trait;

use chappie_kernel::model::ticket::NewTicket;
use chappie_kernel::model::ticket::Ticket;

use chappie_kernel::repository::ticket::TicketRepository;

use super::DatabaseRepository;
use crate::model::ticket::NewTicketTable;

#[async_trait]
impl TicketRepository for DatabaseRepository<Ticket> {
    async fn create(&self, source: NewTicket) -> anyhow::Result<()> {
        let ticket_table: NewTicketTable = source.try_into()?;
        let pool = self.pool.0.clone();
        let mut tx = pool.begin().await.unwrap();

        let _ = sqlx::query_file_as!(
            NewTicketTable,
            "sql/createTicket.sql",
            ticket_table.ticket_title,
            ticket_table.description,
            ticket_table.priority,
            ticket_table.status,
            ticket_table.progress,
            ticket_table.due_date,
            ticket_table.project_id,
            ticket_table.assignee_id,
        )
        .execute(&mut *tx)
        .await;

        tx.commit()
            .await
            .unwrap_or_else(|_| panic!("Commit failed"));

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use chappie_kernel::model::ticket::NewTicket;
    use chappie_kernel::model::Id;
    use chappie_kernel::repository::ticket::TicketRepository;
    use chrono::NaiveDate;
    use ulid::Ulid;

    use super::DatabaseRepository;
    use crate::persistence::database::Db;

    #[tokio::test]
    async fn create_ticket() {
        let db = Db::new().await;
        let repository = DatabaseRepository::new(db);
        let id = Ulid::new();
        let due_date = NaiveDate::parse_from_str("2023-12-31", "%Y-%m-%d").unwrap();

        repository
            .create(NewTicket::new(
                "TestTicket".to_string(),
                "Test Ticket".to_string(),
                9,
                9,
                0,
                due_date,
                1,
                Id::new(id),
            ))
            .await
            .unwrap();
    }
}

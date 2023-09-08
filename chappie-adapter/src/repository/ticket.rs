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
            ticket_table.ticket_type_id,
            ticket_table.description,
            ticket_table.priority,
            ticket_table.status_id,
            ticket_table.progress,
            ticket_table.start_date,
            ticket_table.due_date,
            ticket_table.created_at,
            ticket_table.updated_at,
            ticket_table.parent_ticket_id,
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
        let project_id = Ulid::new();
        let assignee_id = Ulid::new();
        let start_date = NaiveDate::parse_from_str("2023-09-01", "%Y-%m-%d").unwrap();
        let due_date = NaiveDate::parse_from_str("2023-12-31", "%Y-%m-%d").unwrap();

        repository
            .create(NewTicket::new(
                "TestTicket".to_string(),
                1,
                "Test Ticket".to_string(),
                9,
                9,
                0,
                start_date,
                due_date,
                Some(9999),
                Id::new(project_id),
                Id::new(assignee_id),
            ))
            .await
            .unwrap();
    }
}

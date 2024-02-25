use async_trait::async_trait;
use crate::model::ticket::NewTicketTable;
use crate::model::ticket::TicketTable;
use chappie_kernel::model::ticket::NewTicket;
use chappie_kernel::model::ticket::Ticket;
use chappie_kernel::repository::ticket::TicketRepository;
use super::DatabaseRepository;

#[async_trait]
impl TicketRepository for DatabaseRepository<Ticket> {
    async fn create(&self, source: NewTicket) -> anyhow::Result<()> {
        let ticket_table: NewTicketTable = source.try_into()?;

        let query = sqlx::query_file_as!(
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
        );

        self.execute(query).await
    }

    async fn find(&self, id: i32) -> anyhow::Result<Option<Ticket>> {
        let ticket_table = sqlx::query_file_as!(TicketTable, "sql/findTicket.sql", id,)
            .fetch_one(&*self.cloned_connection_pool())
            .await
            .ok();

        match ticket_table {
            Some(ticket) => Ok(Some(ticket.try_into()?)),
            None => Ok(None),
        }
    }

    async fn list(&self) -> anyhow::Result<Option<Vec<Ticket>>> {
        let ticket_table = sqlx::query_file_as!(TicketTable, "sql/getTicketList.sql")
            .fetch_all(&*self.cloned_connection_pool())
            .await
            .ok();

        match ticket_table {
            Some(list) => {
                let ticket_list = list.into_iter().flat_map(|t| t.try_into()).collect();
                Ok(Some(ticket_list))
            }
            None => Ok(None),
        }
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
                Some(99999),
                Id::new(project_id),
                Id::new(assignee_id),
            ))
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn create_ticket_no_parent() {
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
                None,
                Id::new(project_id),
                Id::new(assignee_id),
            ))
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_find_ticket() {
        let db = Db::new().await;
        let repository = DatabaseRepository::new(db);

        let ticket = repository.find(1).await.unwrap();

        assert_eq!(ticket.unwrap().ticket_id, 1);
    }

    #[tokio::test]
    async fn test_ticket_list() {
        let db = Db::new().await;
        let repository = DatabaseRepository::new(db);

        let ticket = repository.list().await.unwrap();

        assert!(ticket.unwrap().len() > 1);
    }
}

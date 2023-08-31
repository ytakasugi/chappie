use chrono::{NaiveDate, NaiveDateTime};

use chappie_kernel::model::ticket::NewTicket;
use sqlx::FromRow;

#[derive(FromRow)]
pub struct TicketTable {
    pub ticket_id: i32,
    pub ticket_title: String,
    pub description: String,
    pub status: i32,
    pub progress: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub due_date: NaiveDate,
    pub project_id: i32,
    // user_id
    pub assignee_id: String,
}

pub struct NewTicketTable {
    pub ticket_title: String,
    pub description: String,
    pub priority: i32,
    pub status: i32,
    pub progress: i32,
    pub due_date: NaiveDate,
    pub project_id: i32,
    // user_id
    pub assignee_id: String,
}

impl TryFrom<NewTicket> for NewTicketTable {
    type Error = anyhow::Error;

    fn try_from(ticket: NewTicket) -> Result<Self, Self::Error> {
        Ok(NewTicketTable {
            ticket_title: ticket.ticket_title,
            description: ticket.description,
            priority: ticket.priority,
            status: ticket.status,
            progress: ticket.progress,
            due_date: ticket.due_date,
            project_id: ticket.project_id,
            assignee_id: ticket.assignee_id.value.to_string(),
        })
    }
}

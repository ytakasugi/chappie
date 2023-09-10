use chrono::{Local, NaiveDate, NaiveDateTime};

use chappie_kernel::model::ticket::{NewTicket, Ticket};
use sqlx::FromRow;

#[derive(FromRow)]
pub struct TicketTable {
    pub ticket_id: i32,
    pub ticket_title: String,
    pub ticket_type_id: i32,
    pub description: String,
    pub priority: i32,
    pub status_id: i32,
    pub progress: i32,
    pub start_date: NaiveDate,
    pub due_date: NaiveDate,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub project_id: String,
    // user_id
    pub assignee_id: String,
}

impl TryFrom<TicketTable> for Ticket {
    type Error = anyhow::Error;

    fn try_from(ticket: TicketTable) -> Result<Self, Self::Error> {
        Ok(Ticket::new(
            ticket.ticket_id,
            ticket.ticket_title,
            ticket.ticket_type_id,
            ticket.description,
            ticket.priority,
            ticket.status_id,
            ticket.progress,
            ticket.start_date,
            ticket.due_date,
            ticket.created_at,
            ticket.updated_at,
            ticket.project_id.try_into()?,
            ticket.assignee_id.try_into()?,
        ))
    }
}

pub struct NewTicketTable {
    pub ticket_title: String,
    pub ticket_type_id: i32,
    pub description: String,
    pub priority: i32,
    pub status_id: i32,
    pub progress: i32,
    pub start_date: NaiveDate,
    pub due_date: NaiveDate,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub project_id: String,
    // user_id
    pub assignee_id: String,
}

impl TryFrom<NewTicket> for NewTicketTable {
    type Error = anyhow::Error;

    fn try_from(ticket: NewTicket) -> Result<Self, Self::Error> {
        Ok(NewTicketTable {
            ticket_title: ticket.ticket_title,
            ticket_type_id: ticket.ticket_type_id,
            description: ticket.description,
            priority: ticket.priority,
            status_id: ticket.status_id,
            progress: ticket.progress,
            start_date: ticket.start_date,
            due_date: ticket.due_date,
            created_at: Local::now().naive_local(),
            updated_at: Local::now().naive_local(),
            project_id: ticket.project_id.value.to_string(),
            assignee_id: ticket.assignee_id.value.to_string(),
        })
    }
}

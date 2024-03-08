use super::helper::convert_str_to_date;
use chappie_kernel::model::ticket::{NewTicket, Ticket};
use derive_new::new;

pub struct TicketView {
    pub ticket_id: i32,
    pub ticket_title: String,
    pub ticket_type_id: i32,
    pub description: String,
    pub priority: i32,
    pub status_id: i32,
    pub progress: i32,
    pub start_date: String,
    pub due_date: String,
    pub parent_ticket_id: Option<i32>,
    pub project_id: String,
    // user_id
    pub assignee_id: String,
}

impl TicketView {
    pub fn new(ticket: Ticket) -> Self {
        Self {
            ticket_id: ticket.ticket_id,
            ticket_title: ticket.ticket_title,
            ticket_type_id: ticket.ticket_type_id,
            description: ticket.description,
            priority: ticket.priority,
            status_id: ticket.status_id,
            progress: ticket.progress,
            start_date: ticket.start_date.to_string(),
            due_date: ticket.due_date.to_string(),
            parent_ticket_id: ticket.parent_ticket_id,
            project_id: ticket.project_id.value.to_string(),
            assignee_id: ticket.assignee_id.value.to_string(),
        }
    }
}

impl From<Ticket> for TicketView {
    fn from(ticket: Ticket) -> Self {
        Self {
            ticket_id: ticket.ticket_id,
            ticket_title: ticket.ticket_title,
            ticket_type_id: ticket.ticket_type_id,
            description: ticket.description,
            priority: ticket.priority,
            status_id: ticket.status_id,
            progress: ticket.progress,
            start_date: ticket.start_date.to_string(),
            due_date: ticket.due_date.to_string(),
            parent_ticket_id: ticket.parent_ticket_id,
            project_id: ticket.project_id.value.to_string(),
            assignee_id: ticket.assignee_id.value.to_string(),
        }
    }
}

#[allow(clippy::too_many_arguments)]
#[derive(new)]
pub struct CreateTicket {
    pub ticket_title: String,
    pub ticket_type_id: i32,
    pub description: String,
    pub priority: i32,
    pub status_id: i32,
    pub progress: i32,
    pub start_date: String,
    pub due_date: String,
    pub parent_ticket_id: Option<i32>,
    pub project_id: String,
    // user_id
    pub assignee_id: String,
}

impl TryFrom<CreateTicket> for NewTicket {
    type Error = anyhow::Error;

    fn try_from(c: CreateTicket) -> Result<Self, Self::Error> {
        Ok(NewTicket::new(
            c.ticket_title,
            c.ticket_type_id,
            c.description,
            c.priority,
            c.status_id,
            c.progress,
            convert_str_to_date(c.start_date)?,
            convert_str_to_date(c.due_date)?,
            c.parent_ticket_id,
            c.project_id.try_into()?,
            c.assignee_id.try_into()?,
        ))
    }
}

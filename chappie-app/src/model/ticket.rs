use chappie_kernel::model::ticket::NewTicket;

use super::helper::convert_str_to_date;
use derive_new::new;

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
    pub project_id: i32,
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
            c.project_id,
            c.assignee_id.try_into()?,
        ))
    }
}

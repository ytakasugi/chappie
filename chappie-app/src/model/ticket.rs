use crate::helper::LocalDateTime;
use chappie_kernel::model::ticket::NewTicket;
use derive_new::new;

#[allow(clippy::too_many_arguments)]
#[derive(new)]
pub struct CreateTicket {
    pub ticket_title: String,
    pub description: String,
    pub priority: i32,
    pub status: i32,
    pub progress: i32,
    pub due_date: LocalDateTime,
    pub project_id: i32,
    // user_id
    pub assignee_id: String,
}

impl TryFrom<CreateTicket> for NewTicket {
    type Error = anyhow::Error;

    fn try_from(c: CreateTicket) -> Result<Self, Self::Error> {
        Ok(NewTicket::new(
            c.ticket_title,
            c.description,
            c.priority,
            c.status,
            c.progress,
            c.due_date.try_into()?,
            c.project_id,
            c.assignee_id.try_into()?,
        ))
    }
}

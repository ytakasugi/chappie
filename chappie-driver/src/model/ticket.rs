use chappie_app::model::ticket::CreateTicket;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct JsonCreateTicket {
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

impl From<JsonCreateTicket> for CreateTicket {
    fn from(t: JsonCreateTicket) -> Self {
        CreateTicket {
            ticket_title: t.ticket_title,
            ticket_type_id: t.ticket_type_id,
            description: t.description,
            priority: t.priority,
            status_id: t.status_id,
            progress: t.progress,
            start_date: t.start_date,
            due_date: t.due_date,
            parent_ticket_id: t.parent_ticket_id,
            project_id: t.project_id,
            assignee_id: t.assignee_id,
        }
    }
}

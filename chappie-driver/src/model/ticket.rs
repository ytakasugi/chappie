use chappie_app::model::ticket::CreateTicket;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct JsonCreateTicket {
    pub ticket_title: String,
    pub description: String,
    pub priority: i32,
    pub status: i32,
    pub progress: i32,
    pub due_date: String,
    pub project_id: i32,
    // user_id
    pub assignee_id: String,
}

impl From<JsonCreateTicket> for CreateTicket {
    fn from(t: JsonCreateTicket) -> Self {
        CreateTicket {
            ticket_title: t.ticket_title,
            description: t.description,
            priority: t.priority,
            status: t.status,
            progress: t.progress,
            due_date: t.due_date,
            project_id: t.project_id,
            assignee_id: t.assignee_id,
        }
    }
}

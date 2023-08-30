use chrono::{NaiveDate, NaiveDateTime};
use derive_new::new;

use super::Id;

#[allow(clippy::too_many_arguments)]
#[derive(new, Debug, PartialEq, Eq)]
pub struct Ticket {
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
    pub assignee_id: Id<Ticket>,
}

#[allow(clippy::too_many_arguments)]
#[derive(new, Debug)]
pub struct NewTicket {
    pub ticket_title: String,
    pub description: String,
    pub priority: i32,
    pub status: i32,
    pub progress: i32,
    pub due_date: NaiveDate,
    pub project_id: i32,
    // user_id
    pub assignee_id: Id<Ticket>,
}

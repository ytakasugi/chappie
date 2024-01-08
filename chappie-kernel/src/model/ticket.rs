use chrono::{NaiveDate, NaiveDateTime};
use derive_new::new;

use super::{project::Project, user::User, Id};

#[allow(clippy::too_many_arguments)]
#[derive(new, Debug, PartialEq, Eq)]
pub struct Ticket {
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
    pub updated_at: Option<NaiveDateTime>,
    pub parent_ticket_id: Option<i32>,
    pub project_id: Id<Project>,
    // user_id
    pub assignee_id: Id<User>,
}

#[allow(clippy::too_many_arguments)]
#[derive(new, Debug)]
pub struct NewTicket {
    pub ticket_title: String,
    pub ticket_type_id: i32,
    pub description: String,
    pub priority: i32,
    pub status_id: i32,
    pub progress: i32,
    pub start_date: NaiveDate,
    pub due_date: NaiveDate,
    pub parent_ticket_id: Option<i32>,
    pub project_id: Id<Project>,
    // user_id
    pub assignee_id: Id<User>,
}

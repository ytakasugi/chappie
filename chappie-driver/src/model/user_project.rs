use chappie_app::model::user_project::CreateUserProject;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct JsonCreateUserProject {
    pub user_id: String,
    pub project_id: String,
    pub role: String,
}

impl From<JsonCreateUserProject> for CreateUserProject {
    fn from(u: JsonCreateUserProject) -> Self {
        CreateUserProject {
            user_id: u.user_id,
            project_id: u.project_id,
            role: u.role,
        }
    }
}

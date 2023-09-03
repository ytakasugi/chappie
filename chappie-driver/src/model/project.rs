use chappie_app::model::project::CreateProject;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct JsonCreateProject {
    pub project_name: String,
    pub description: String,
    pub manager_id: String,
}

impl From<JsonCreateProject> for CreateProject {
    fn from(p: JsonCreateProject) -> Self {
        CreateProject {
            project_name: p.project_name,
            description: p.description,
            manager_id: p.manager_id,
        }
    }
}

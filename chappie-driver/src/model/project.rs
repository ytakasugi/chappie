use chappie_app::model::project::CreateProject;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct JsonCreateProject {
    pub project_title: String,
    pub project_name: String,
    pub description: String,
    pub parent_project_id: Option<String>,
    pub manager_id: String,
}

impl From<JsonCreateProject> for CreateProject {
    fn from(p: JsonCreateProject) -> Self {
        CreateProject {
            project_title: p.project_title,
            project_name: p.project_name,
            description: p.description,
            parent_project_id: p.parent_project_id,
            manager_id: p.manager_id,
        }
    }
}

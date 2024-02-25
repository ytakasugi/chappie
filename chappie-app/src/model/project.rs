use super::helper::convert_string_to_id;
use chappie_kernel::model::{project::NewProject, Id};
use derive_new::new;

#[derive(new)]
pub struct CreateProject {
    pub project_title: String,
    pub project_name: String,
    pub description: String,
    pub parent_project_id: Option<String>,
    pub manager_id: String,
}

impl TryFrom<CreateProject> for NewProject {
    type Error = anyhow::Error;

    fn try_from(c: CreateProject) -> Result<Self, Self::Error> {
        let project_id = Id::gen();

        Ok(NewProject::new(
            project_id,
            c.project_title,
            c.project_name,
            c.description,
            convert_string_to_id(c.parent_project_id),
            c.manager_id.try_into()?,
        ))
    }
}

use chappie_kernel::model::project::NewProject;
use derive_new::new;

#[derive(new)]
pub struct CreateProject {
    pub project_name: String,
    pub description: String,
    pub parent_project_id: Option<i32>,
    pub manager_id: String,
}

impl TryFrom<CreateProject> for NewProject {
    type Error = anyhow::Error;

    fn try_from(c: CreateProject) -> Result<Self, Self::Error> {
        Ok(NewProject::new(
            c.project_name,
            c.description,
            c.parent_project_id,
            c.manager_id.try_into()?,
        ))
    }
}

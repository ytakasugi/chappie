use chappie_kernel::model::user_project::NewUserProject;
use derive_new::new;

#[derive(new)]
pub struct CreateUserProject {
    pub user_id: String,
    pub project_id: i32,
    pub role: String,
}

impl TryFrom<CreateUserProject> for NewUserProject {
    type Error = anyhow::Error;

    fn try_from(c: CreateUserProject) -> Result<Self, Self::Error> {
        Ok(NewUserProject::new(
            c.user_id.try_into()?,
            c.project_id,
            c.role,
        ))
    }
}

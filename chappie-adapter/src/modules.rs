use chappie_kernel::model::{project::Project, user::User};
use chappie_kernel::repository::project::ProjectRepository;
use chappie_kernel::repository::user::UserRepository;

use crate::{persistence::database::Db, repository::DatabaseRepository};

pub struct RepositoriesModule {
    user_repository: DatabaseRepository<User>,
    project_repository: DatabaseRepository<Project>,
}

pub trait RepositoriesModuleExt {
    type UserRepo: UserRepository;
    type ProjectRepo: ProjectRepository;

    fn user_repository(&self) -> &Self::UserRepo;
    fn project_repository(&self) -> &Self::ProjectRepo;
}

impl RepositoriesModuleExt for RepositoriesModule {
    type UserRepo = DatabaseRepository<User>;
    type ProjectRepo = DatabaseRepository<Project>;

    fn user_repository(&self) -> &Self::UserRepo {
        &self.user_repository
    }

    fn project_repository(&self) -> &Self::ProjectRepo {
        &self.project_repository
    }
}

impl RepositoriesModule {
    pub fn new(db: Db) -> Self {
        let user_repository = DatabaseRepository::new(db.clone());
        let project_repository = DatabaseRepository::new(db.clone());

        Self {
            user_repository,
            project_repository,
        }
    }
}

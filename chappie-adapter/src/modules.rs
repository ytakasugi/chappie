use chappie_kernel::model::ticket::Ticket;
use chappie_kernel::model::{project::Project, user::User, user_project::UserProject};
use chappie_kernel::repository::project::ProjectRepository;
use chappie_kernel::repository::ticket::TicketRepository;
use chappie_kernel::repository::user::UserRepository;
use chappie_kernel::repository::user_project::UserProjectRepository;

use crate::{persistence::database::Db, repository::DatabaseRepository};

pub struct RepositoriesModule {
    user_repository: DatabaseRepository<User>,
    project_repository: DatabaseRepository<Project>,
    ticket_repository: DatabaseRepository<Ticket>,
    user_project_repository: DatabaseRepository<UserProject>,
}

pub trait RepositoriesModuleExt {
    type UserRepo: UserRepository;
    type ProjectRepo: ProjectRepository;
    type TicketRepo: TicketRepository;
    type UserProjectRepo: UserProjectRepository;

    fn user_repository(&self) -> &Self::UserRepo;
    fn project_repository(&self) -> &Self::ProjectRepo;
    fn ticket_repository(&self) -> &Self::TicketRepo;
    fn user_project_repository(&self) -> &Self::UserProjectRepo;
}

impl RepositoriesModuleExt for RepositoriesModule {
    type UserRepo = DatabaseRepository<User>;
    type ProjectRepo = DatabaseRepository<Project>;
    type TicketRepo = DatabaseRepository<Ticket>;
    type UserProjectRepo = DatabaseRepository<UserProject>;

    fn user_repository(&self) -> &Self::UserRepo {
        &self.user_repository
    }

    fn project_repository(&self) -> &Self::ProjectRepo {
        &self.project_repository
    }

    fn ticket_repository(&self) -> &Self::TicketRepo {
        &self.ticket_repository
    }

    fn user_project_repository(&self) -> &Self::UserProjectRepo {
        &self.user_project_repository
    }
}

impl RepositoriesModule {
    pub fn new(db: Db) -> Self {
        let user_repository = DatabaseRepository::new(db.clone());
        let project_repository = DatabaseRepository::new(db.clone());
        let ticket_repository = DatabaseRepository::new(db.clone());
        let user_project_repository = DatabaseRepository::new(db.clone());

        Self {
            user_repository,
            project_repository,
            ticket_repository,
            user_project_repository,
        }
    }
}

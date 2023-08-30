use chappie_kernel::model::ticket::Ticket;
use chappie_kernel::model::{project::Project, user::User};
use chappie_kernel::repository::project::ProjectRepository;
use chappie_kernel::repository::ticket::TicketRepository;
use chappie_kernel::repository::user::UserRepository;

use crate::{persistence::database::Db, repository::DatabaseRepository};

pub struct RepositoriesModule {
    user_repository: DatabaseRepository<User>,
    project_repository: DatabaseRepository<Project>,
    ticket_repository: DatabaseRepository<Ticket>,
}

pub trait RepositoriesModuleExt {
    type UserRepo: UserRepository;
    type ProjectRepo: ProjectRepository;
    type TicketRepo: TicketRepository;

    fn user_repository(&self) -> &Self::UserRepo;
    fn project_repository(&self) -> &Self::ProjectRepo;
    fn ticket_repository(&self) -> &Self::TicketRepo;
}

impl RepositoriesModuleExt for RepositoriesModule {
    type UserRepo = DatabaseRepository<User>;
    type ProjectRepo = DatabaseRepository<Project>;
    type TicketRepo = DatabaseRepository<Ticket>;

    fn user_repository(&self) -> &Self::UserRepo {
        &self.user_repository
    }

    fn project_repository(&self) -> &Self::ProjectRepo {
        &self.project_repository
    }

    fn ticket_repository(&self) -> &Self::TicketRepo {
        &self.ticket_repository
    }
}

impl RepositoriesModule {
    pub fn new(db: Db) -> Self {
        let user_repository = DatabaseRepository::new(db.clone());
        let project_repository = DatabaseRepository::new(db.clone());
        let ticket_repository = DatabaseRepository::new(db.clone());

        Self {
            user_repository,
            project_repository,
            ticket_repository,
        }
    }
}

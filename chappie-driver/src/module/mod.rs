use std::sync::Arc;

use chappie_adapter::{
    modules::{RepositoriesModule, RepositoriesModuleExt},
    persistence::database::Db,
};

use chappie_app::usecase::project::ProjectUseCase;
use chappie_app::usecase::ticket::TicketUseCase;
use chappie_app::usecase::user::UserUseCase;
use chappie_app::usecase::user_project::UserProjectUseCase;

pub struct Modules {
    user_usecase: UserUseCase<RepositoriesModule>,
    project_usecase: ProjectUseCase<RepositoriesModule>,
    ticket_usecase: TicketUseCase<RepositoriesModule>,
    user_project_usecase: UserProjectUseCase<RepositoriesModule>,
}

pub trait ModulesExt {
    type RepositoriesModule: RepositoriesModuleExt;

    fn user_usecase(&self) -> &UserUseCase<Self::RepositoriesModule>;
    fn project_usecase(&self) -> &ProjectUseCase<Self::RepositoriesModule>;
    fn ticket_usecase(&self) -> &TicketUseCase<Self::RepositoriesModule>;
    fn user_project_usecase(&self) -> &UserProjectUseCase<Self::RepositoriesModule>;
}

impl ModulesExt for Modules {
    type RepositoriesModule = RepositoriesModule;

    fn user_usecase(&self) -> &UserUseCase<Self::RepositoriesModule> {
        &self.user_usecase
    }

    fn project_usecase(&self) -> &ProjectUseCase<Self::RepositoriesModule> {
        &self.project_usecase
    }

    fn ticket_usecase(&self) -> &TicketUseCase<Self::RepositoriesModule> {
        &self.ticket_usecase
    }

    fn user_project_usecase(&self) -> &UserProjectUseCase<Self::RepositoriesModule> {
        &self.user_project_usecase
    }
}

impl Modules {
    pub async fn new() -> Modules {
        let db = Db::new().await;
        let repositories_module = Arc::new(RepositoriesModule::new(db));

        let user_usecase = UserUseCase::new(repositories_module.clone());
        let project_usecase = ProjectUseCase::new(repositories_module.clone());
        let ticket_usecase = TicketUseCase::new(repositories_module.clone());
        let user_project_usecase = UserProjectUseCase::new(repositories_module.clone());

        Self {
            user_usecase,
            ticket_usecase,
            project_usecase,
            user_project_usecase,
        }
    }
}

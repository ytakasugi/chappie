use std::sync::Arc;

use chappie_adapter::{
    modules::{RepositoriesModule, RepositoriesModuleExt},
    persistence::database::Db,
};

use chappie_app::usecase::user::UserUseCase;

pub struct Modules {
    user_usecase: UserUseCase<RepositoriesModule>,
}

pub trait ModulesExt {
    type RepositoriesModule: RepositoriesModuleExt;

    fn user_usecase(&self) -> &UserUseCase<Self::RepositoriesModule>;
}

impl ModulesExt for Modules {
    type RepositoriesModule = RepositoriesModule;

    fn user_usecase(&self) -> &UserUseCase<Self::RepositoriesModule> {
        &self.user_usecase
    }
}

impl Modules {
    pub async fn new() -> Modules {
        let db = Db::new().await;
        let repositories_module = Arc::new(RepositoriesModule::new(db));

        let user_usecase = UserUseCase::new(repositories_module.clone());
        Self { user_usecase }
    }
}

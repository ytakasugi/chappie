use std::sync::Arc;

use derive_new::new;

use chappie_adapter::modules::RepositoriesModuleExt;
use chappie_kernel::repository::ticket::TicketRepository;

use crate::model::ticket::{CreateTicket, TicketView};

#[derive(new)]
pub struct TicketUseCase<R: RepositoriesModuleExt> {
    repositories: Arc<R>,
}

impl<R: RepositoriesModuleExt> TicketUseCase<R> {
    pub async fn create(&self, source: CreateTicket) -> anyhow::Result<()> {
        self.repositories
            .ticket_repository()
            .create(source.try_into()?)
            .await
    }

    pub async fn find(&self, id: i32) -> anyhow::Result<Option<TicketView>> {
        let ticket = self.repositories.ticket_repository().find(id).await?;

        match ticket {
            Some(ticket) => Ok(Some(TicketView::new(ticket))),
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use crate::model::ticket::CreateTicket;
    use chappie_adapter::{modules::RepositoriesModule, persistence::database::Db};

    use super::TicketUseCase;

    #[tokio::test]
    async fn test_ticket_usecase_create() {
        let modules = RepositoriesModule::new(Db::new().await);
        let usecase = TicketUseCase::new(Arc::new(modules));

        let source = CreateTicket::new(
            "TestUseCaseTicket".to_string(),
            1,
            "TestUseCaseTicket".to_string(),
            9,
            9,
            0,
            "2023-09-01".to_string(),
            "2023-12-31".to_string(),
            Some(99999),
            "01H95VREP370GZ080BBH4EZQ6J".to_string(),
            "01H95VREP370GZ080BBH4EZQ6J".to_string(),
        );

        usecase.create(source).await.unwrap();
    }

    #[tokio::test]
    async fn test_ticket_usecase_create_no_parent() {
        let modules = RepositoriesModule::new(Db::new().await);
        let usecase = TicketUseCase::new(Arc::new(modules));

        let source = CreateTicket::new(
            "TestUseCaseTicket".to_string(),
            1,
            "TestUseCaseTicket".to_string(),
            9,
            9,
            0,
            "2023-09-01".to_string(),
            "2023-12-31".to_string(),
            None,
            "01H95VREP370GZ080BBH4EZQ6J".to_string(),
            "01H95VREP370GZ080BBH4EZQ6J".to_string(),
        );

        usecase.create(source).await.unwrap();
    }

    #[tokio::test]
    async fn test_ticket_usecase_find() {
        let modules = RepositoriesModule::new(Db::new().await);
        let usecase = TicketUseCase::new(Arc::new(modules));

        usecase.find(1).await.unwrap();
    }
}

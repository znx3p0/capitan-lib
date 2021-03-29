use async_trait::async_trait;

#[async_trait]
pub trait Reactor {
    type Metadata: Sync + Send;
    type Services;

    async fn init(meta: Self::Metadata, services: Self::Services) -> Self;
}

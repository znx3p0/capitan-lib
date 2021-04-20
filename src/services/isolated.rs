use std::any::Any;

use anyhow::Result as Res;
use async_trait::async_trait;

use super::{
    dynamic::DynamicIsolatedService,
    reactor::{IsolatedReactor, IsolatedReactorTrait},
};

/// isolated services are services that can't be accessed from the outside.
/// they are helpful as root services, services that hold other services.
#[async_trait]
pub trait IsolatedService: Any {
    /// only runs once at the start
    async fn init(&mut self) -> Res<()>;
    /// loops
    async fn main(&mut self) -> Res<()>;
    /// runs after main if main or repeat did not return errors
    async fn repeat(&mut self) -> Res<()>;
    /// runs after main if main returned an error
    async fn catch(&mut self, error: anyhow::Error) -> Res<()>;
    /// run if catch was not successful
    async fn abort(&mut self, error: anyhow::Error) -> Res<()>;

    fn to_dyn(self) -> DynamicIsolatedService
    where
        Self: Sized + Sync + Send,
    {
        DynamicIsolatedService(Box::new(self))
    }
}

use crate::{ignore_print_result, print_err};

#[async_trait]
impl<T: IsolatedService + Send + Sync> IsolatedReactorTrait<T> for IsolatedReactor {
    async fn spawn_service(&self, mut service: T, id: usize) -> Res<()> {
        let services = self.services.clone();
        let channel = self.notifier_channel.0.clone();
        let handle = tokio::spawn(async move {
            if let Err(err) = service.init().await {
                let alive = {
                    let mut services = services.write().await;
                    services.remove(&id);
                    services.len() == 0
                };
                if let Err(e) = service.abort(err).await {
                    channel.send(alive).ok();
                    return Err(e);
                };
                channel.send(alive).ok();
            }

            let err = loop {
                if let Err(e) = service.main().await {
                    ignore_print_result!(service.catch(e).await, e, id);
                }
                if let Err(e) = service.repeat().await {
                    ignore_print_result!(service.catch(e).await, e, id);
                }
            };

            let alive = {
                let mut services = services.write().await;
                services.remove(&id);
                services.len() == 0
            };

            if let Err(e) = service.abort(err).await {
                channel.send(alive).ok();
                return Err(e);
            };

            channel.send(alive).ok();

            Ok(())
        });

        {
            let mut services = self.services.write().await;
            services.insert(id, handle);
        }
        Ok(())
    }
}

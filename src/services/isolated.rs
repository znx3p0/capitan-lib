use std::{any::Any, sync::Arc};

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
    async fn abort(&mut self) -> Res<()>;

    fn to_dyn(self) -> DynamicIsolatedService
    where
        Self: Sized + Sync + Send,
    {
        DynamicIsolatedService(Box::new(self))
    }
}

use crate::catch_err;
use crate::print_err;

#[async_trait]
impl<T: IsolatedService + Send + Sync> IsolatedReactorTrait<T> for IsolatedReactor {
    async fn spawn_service(&self, mut service: T, id: usize) -> Res<()> {
        let handle = tokio::spawn(async move {
            catch_err!(service.init().await);
            loop {
                if let Err(e) = service.main().await {
                    match service.catch(e).await {
                        Ok(_) => log::info!("successfully catching service with id {}", id),
                        Err(e) => {
                            print_err!(e);
                            break;
                        }
                    }
                }
                if let Err(e) = service.repeat().await {
                    match service.catch(e).await {
                        Ok(_) => log::info!("successfully catching service with id {}", id),
                        Err(e) => {
                            print_err!(e);
                            break;
                        }
                    }
                }
            }
            catch_err!(service.abort().await);
            Ok(())
        });

        self.services.write().await.push(handle);
        Ok(())
    }
}

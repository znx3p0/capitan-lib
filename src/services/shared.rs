use super::{dynamic::DynamicSharedService, prelude::SharedR, SharedReactor};
use anyhow::Result as Res;
use async_trait::async_trait;
use std::{
    any::Any,
    sync::{Arc, Weak},
};

#[async_trait]
pub trait SharedService: Any {
    /// only runs once at the start
    async fn init(&self) -> Res<()>;
    /// loops
    async fn main(&self) -> Res<()>;
    /// runs after main if main or repeat did not return errors
    async fn repeat(&self) -> Res<()>;
    /// runs after main if main returned an error
    async fn catch(&self, error: anyhow::Error) -> Res<()>;
    /// run if catch was not successful
    async fn abort(&self, error: anyhow::Error) -> Res<()>;

    fn to_dyn(self) -> (DynamicSharedService, Weak<Self>)
    where
        Self: Sized + Sync + Send,
    {
        let p = Arc::new(self);
        let s = Arc::downgrade(&p);
        (DynamicSharedService(p), s)
    }
}

use crate::{ignore_print_result, print_err};

#[async_trait]
impl<T: SharedService + Send + Sync> SharedR<T> for SharedReactor<T> {
    async fn spawn_service(&self, service: T, id: usize) -> Res<()> {
        let service = Arc::new(service);
        let services = self.services.clone();
        let channel = self.notifier_channel.0.clone();
        let p = service.clone();
        let handle = tokio::spawn(async move {
            if let Err(err) = p.init().await {
                let alive = {
                    let mut services = services.write().await;
                    services.remove(&id);
                    services.len() == 0
                };
                if let Err(e) = p.abort(err).await {
                    channel.send(alive).ok();
                    return Err(e);
                };
                channel.send(alive).ok();
            }

            let err = loop {
                if let Err(e) = p.main().await {
                    ignore_print_result!(p.catch(e).await, e, id);
                }
                if let Err(e) = p.repeat().await {
                    ignore_print_result!(p.catch(e).await, e, id);
                }
            };

            let alive = {
                let mut services = services.write().await;
                services.remove(&id);
                services.len() == 0
            };

            if let Err(e) = p.abort(err).await {
                channel.send(alive).ok();
                return Err(e);
            };

            channel.send(alive).ok();

            Ok(())
        });

        {
            let mut services = self.services.write().await;
            services.insert(id, Arc::new((service, handle)));
        }

        Ok(())
    }

    async fn get_service(&self, id: usize) -> Option<Weak<T>> {
        Some(Arc::downgrade(&self.services.read().await.get(&id)?.0))
    }
}

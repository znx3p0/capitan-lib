use super::{dynamic::DynamicSharedService, prelude::SharedR, SharedReactor};
use anyhow::Result as Res;
use async_trait::async_trait;
use std::{any::Any, sync::Arc};
use tokio::sync::RwLock;

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
    async fn abort(&self) -> Res<()>;

    fn to_dyn(self) -> DynamicSharedService
    where
        Self: Sized + Sync + Send,
    {
        DynamicSharedService(Box::new(self))
    }
}

use crate::{print_err, yields};

#[async_trait]
impl<T: SharedService + Send + Sync> SharedR<T> for SharedReactor<T> {
    async fn spawn_service(&self, service: T, id: usize) -> Res<()> {
        let service = Arc::new(RwLock::new(service));
        let p = service.clone();
        let services = self.services.clone();
        let handle = tokio::spawn(async move {
            yields! {
                let d = p.write().await;
                if let Err(e) = d.init().await {
                    print_err!(e);
                    return Err(e)
                }
            };
            loop {
                // mem drops are used so the rwlock can be usable externally.
                let mut d = p.write().await;
                match d.main().await {
                    Ok(_) => {
                        std::mem::drop(d);
                        let mut d = p.write().await;
                        match d.repeat().await {
                            Ok(_) => std::mem::drop(d),
                            Err(e) => {
                                std::mem::drop(d);
                                let mut d = p.write().await;
                                match d.catch(e).await {
                                    Ok(_) => std::mem::drop(d),
                                    Err(e) => {
                                        print_err!(e);
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        std::mem::drop(d);
                        let d = p.write().await;
                        match d.catch(e).await {
                            Ok(_) => {}
                            Err(e) => {
                                print_err!(e);
                                break;
                            }
                        }
                    }
                }
            }

            yields! {
                let d = p.write().await;
                if let Err(e) = d.abort().await {
                    print_err!(e);
                    return Err(e)
                }
            }

            let mut services = services.write().await;
            services.remove(id);

            Ok(())
        });

        let mut services = self.services.write().await;
        services.push(Arc::new((id, service, handle)));

        Ok(())
    }
}

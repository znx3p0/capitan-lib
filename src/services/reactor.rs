use std::sync::Arc;

use anyhow::Result as Res;
use async_trait::async_trait;
use tokio::{sync::RwLock, task::JoinHandle};

use crate::print_err;

use super::{IsolatedService, SharedService};

#[async_trait]
pub trait SharedReactorTrait<T: SharedService> {
    async fn spawn_service(&self, service: T, id: usize) -> Res<()>;
}

#[async_trait]
pub trait IsolatedReactorTrait<T: IsolatedService> {
    async fn spawn_service(&self, service: T, id: usize) -> Res<()>;
}

pub struct IsolatedReactor {
    pub services: Arc<RwLock<Vec<JoinHandle<Res<()>>>>>,
}

impl IsolatedReactor {
    pub fn new() -> Self {
        Self {
            services: Arc::new(RwLock::new(vec![])),
        }
    }
    pub async fn wait_all(&mut self) {
        loop {
            let mut sr = self.services.write().await;
            match sr.get_mut(0) {
                Some(s) => {
                    if let Err(e) = s.await {
                        print_err!(e)
                    }
                }
                None => break,
            }
        }
    }
}

pub struct SharedReactor<T: SharedService + Send + Sync> {
    pub services: Arc<RwLock<Vec<Arc<(/*id*/ usize, Arc<RwLock<T>>, JoinHandle<Res<()>>)>>>>,
}

impl<T: SharedService + Send + Sync> SharedReactor<T> {
    pub fn new() -> Self {
        Self {
            services: Arc::new(RwLock::new(vec![])),
        }
    }
}

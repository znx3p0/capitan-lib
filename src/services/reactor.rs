use std::{
    collections::HashMap,
    sync::{Arc, Weak},
};

use anyhow::Result as Res;
use async_trait::async_trait;
use tokio::{sync::RwLock, task::JoinHandle};

use super::{IsolatedService, SharedService};

#[async_trait]
pub trait SharedReactorTrait<T: SharedService> {
    async fn spawn_service(&self, service: T, id: usize) -> Res<()>;
    async fn get_service(&self, id: usize) -> Option<Weak<T>>;
}

#[async_trait]
pub trait IsolatedReactorTrait<T: IsolatedService> {
    async fn spawn_service(&self, service: T, id: usize) -> Res<()>;
}

use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

/// Contains and runs isolated services
pub struct IsolatedReactor {
    pub services: Arc<RwLock<HashMap<usize, JoinHandle<Res<()>>>>>,
    pub notifier_channel: (UnboundedSender<bool>, UnboundedReceiver<bool>),
}

impl IsolatedReactor {
    pub fn new() -> Self {
        Self {
            services: Arc::new(RwLock::new(HashMap::new())),
            notifier_channel: unbounded_channel(),
        }
    }
    pub async fn wait_all(&mut self) {
        loop {
            match self.notifier_channel.1.recv().await {
                Some(done) => {
                    if done {
                        self.notifier_channel.1.close()
                    }
                }
                None => break,
            }
        }
    }
}

/// Contains and runs shared services
pub struct SharedReactor<T: SharedService + Send + Sync> {
    pub services: Arc<RwLock<HashMap<usize, Arc<(Arc<T>, JoinHandle<Res<()>>)>>>>,
    pub notifier_channel: (UnboundedSender<bool>, UnboundedReceiver<bool>),
}

impl<T: SharedService + Send + Sync> SharedReactor<T> {
    pub fn new() -> Self {
        Self {
            services: Arc::new(RwLock::new(HashMap::new())),
            notifier_channel: unbounded_channel(),
        }
    }
    pub async fn wait_all(&mut self) {
        loop {
            match self.notifier_channel.1.recv().await {
                Some(done) => {
                    if done {
                        self.notifier_channel.1.close()
                    }
                }
                None => break,
            }
        }
    }
}

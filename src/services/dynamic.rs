use async_trait::async_trait;

use anyhow::Result as Res;

use super::{isolated::IsolatedService, shared::SharedService};

pub struct DynamicIsolatedService(pub(crate) Box<(dyn IsolatedService + Send + Sync)>);
pub struct DynamicSharedService(pub(crate) Box<(dyn SharedService + Send + Sync)>);

#[async_trait]
impl IsolatedService for DynamicIsolatedService {
    async fn init(&mut self) -> Res<()> {
        self.0.init().await?;
        Ok(())
    }

    async fn main(&mut self) -> Res<()> {
        self.0.main().await?;
        Ok(())
    }

    async fn repeat(&mut self) -> Res<()> {
        self.0.repeat().await?;
        Ok(())
    }

    async fn catch(&mut self, error: anyhow::Error) -> Res<()> {
        self.0.catch(error).await?;
        Ok(())
    }

    async fn abort(&mut self) -> Res<()> {
        self.0.abort().await?;
        Ok(())
    }
}

#[async_trait]
impl SharedService for DynamicSharedService {
    async fn init(&self) -> Res<()> {
        self.0.init().await?;
        Ok(())
    }

    async fn main(&self) -> Res<()> {
        self.0.main().await?;
        Ok(())
    }

    async fn repeat(&self) -> Res<()> {
        self.0.repeat().await?;
        Ok(())
    }

    async fn catch(&self, error: anyhow::Error) -> Res<()> {
        self.0.catch(error).await?;
        Ok(())
    }

    async fn abort(&self) -> Res<()> {
        self.0.abort().await?;
        Ok(())
    }
}

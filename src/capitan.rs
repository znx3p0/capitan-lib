

use crate::{Res, reactor::Reactor};
use async_trait::async_trait;
use tokio::spawn;



#[macro_export]
macro_rules! capitan {
    () => {};
    (metadata: $i:expr, services: [$($service: ident,)+]) => {
        {
            let meta = std::sync::Arc::new($i);
            let services = {
                let mut v = vec![];
                $(
                    let a = meta.clone();
                    let p: tokio::task::JoinHandle<Res<()>> = tokio::spawn(async move {
                        let p = $service;
                        p.init(a.clone()).await?;
                        loop {
                            p.main(a.clone()).await?;
                            p.fallback(a.clone()).await?;
                        }
                    });
                    v.push(p);
                )+
                v
            };
            StdReactor::init(meta.clone(), services).await;
        }
    }
    /*
    still a sketch
    usage should be the following:
        capitan! {
            metadata: AtomicU32::new(0),
            services: [
                HTTP,
                LB,
                ETC
            ]
        }
    */
}

pub struct StdReactor <T> {
    pub metadata: T,
    pub services: Vec<tokio::task::JoinHandle<Res<()>>>,
}

#[async_trait]
impl <T: Sync + Send> Reactor for StdReactor <T> {
    type Metadata = T;
    type Services = Vec<tokio::task::JoinHandle<Res<()>>>;

    async fn init(metadata: Self::Metadata, services: Self::Services) -> Self {
        StdReactor {
            metadata,
            services,
        }
    }
}




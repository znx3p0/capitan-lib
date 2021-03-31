#![allow(dead_code)]

use std::{any::Any, collections::HashMap, sync::Arc};

use async_trait::async_trait;
use tokio::{spawn, sync::RwLock, task::JoinHandle};

use anyhow::Result as Res;

macro_rules! print_err {
    ($err: ident) => {
        log::error!(
            "catched error {:?} in thread {:?}, {}:{}:{}",
            $err,
            std::thread::current().name().unwrap_or_default(),
            file!(),
            line!(),
            column!()
        );
    };
}

#[async_trait]
pub trait Service: Any {
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
    fn to_dyn(self) -> DynamicService where Self: Sized + Sync + Send {
        DynamicService(Box::new(self))
    }
}

pub struct DynamicService(Box<(dyn Service + Send + Sync)>);

impl std::convert::From<Box<dyn Service + Send + Sync>> for DynamicService {
    fn from(s: Box<dyn Service + Send + Sync>) -> Self {
        DynamicService(s)
    }
}

#[async_trait]
impl Service for DynamicService {
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

pub struct Reactor <T: Service + Send + Sync> {
    pub services: Arc<RwLock<HashMap</* id */ String, Arc<(Arc<RwLock<T>>, JoinHandle<Res<()>>)>>>>
}

macro_rules! yields {
    ($($b: tt)*) => {
        async {$($b)*;Ok(())}.await?;
    };
}

impl <T: Service + Send + Sync> Reactor <T> {
    pub fn new() -> Self {
        Self { services: Arc::new(RwLock::new(HashMap::new())) }
    }

    pub async fn spawn_service(&mut self, service_id: &str, service: T) -> Res<()> {
        
        
        let service = Arc::new(RwLock::new(service));
        
        let p = service.clone();
        let services =self.services.clone();
        let sid = service_id.to_string();

        let p = spawn(async move {

            yields! {
                let mut d = p.write().await;
                if let Err(e) = d.init().await {
                    print_err!(e);
                    return Err(e)
                }
            };

            // async {
            //     let mut d = p.write().await;
            //     if let Err(e) = d.init().await {
            //         print_err!(e);
            //         return Err(e)
            //     }
            //     Ok(())
            // }.await?;

            
            loop {

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
                                    },
                                }
                            }
                        }
                    }
                    Err(e) => {
                        std::mem::drop(d);
                        let mut d = p.write().await;
                        match d.catch(e).await {
                            Ok(_) => {},
                            Err(e) => {
                                print_err!(e);
                                break;
                            },
                        }
                    }
        
                }

            }

            let mut services = services.write().await;
            services.remove(&sid);

            yields! {
                let mut d = p.write().await;   
                if let Err(e) = d.abort().await {
                    print_err!(e);
                    return Err(e)
                }
            }
            
            Ok(())
        });
        
        let mut services = self.services.write().await;
        services.insert(service_id.to_string(), Arc::new((service, p)));

        Ok(())
    }
}





















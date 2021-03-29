use crate::{reactor::Reactor, Res};
use async_trait::async_trait;


/// helper macro to build highly-available services
/// see README in repo for details
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
                        
                        if let Err(err) = p.init(&a).await {
                            println!(
                                "init failed in error {:?}, thread {:?}, {}:{}:{}",
                                err,
                                std::thread::current().name().unwrap_or_default(),
                                file!(),
                                line!(),
                                column!()
                            );
                            return Err(err)
                        };

                        loop {
                            if let Err(err) = p.main(&a).await {
                                println!(
                                    "catching error {:?} in thread {:?}, {}:{}:{}",
                                    err,
                                    std::thread::current().name().unwrap_or_default(),
                                    file!(),
                                    line!(),
                                    column!()
                                );
                            };
                            if let Err(err) = p.fallback(&a).await {
                                println!(
                                    "fallback failed in error {:?}, thread {:?}, {}:{}:{}",
                                    err,
                                    std::thread::current().name().unwrap_or_default(),
                                    file!(),
                                    line!(),
                                    column!()
                                );
                                return Err(err)
                            };
                        }
                    });
                    v.push(p);
                )+
                v
            };
            StdReactor::init(meta.clone(), services).await
        }
    }
    /*
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

/// private macro used for the steer macro
#[macro_export(crate)]
macro_rules! steer_build_m {
    ($name:ident)=>{};
    ( $name:ident rx_handshake; $($tail:tt)*) => {
        $name.stream.receive_handshake().await?;
        $crate::steer_build_m!( $name $($tail)*);
    };
    ( $name:ident tx_handshake; $($tail:tt)*) => {
        $name.stream.send_handshake().await?;
        $crate::steer_build_m!( $name $($tail)*);
    };

    ( $name:ident rx_keepalive; $($tail:tt)*) => {
        $name.stream.receive_keepalive().await?;
        $crate::steer_build_m!( $name $($tail)*);
    };
    ( $name:ident tx_keepalive; $($tail:tt)*) => {
        $name.stream.send_keepalive().await?;
        $crate::steer_build_m!( $name $($tail)*);
    };

    ( $name: ident peer $b:block; $($tail:tt)*) => {
        $crate::steer_build_m!( $name $($tail)*);
    };
    ( $name: ident both $b:block; $($tail:tt)*) => {
        $b;$crate::steer_build_m!( $name $($tail)*);
    };
    ( $name: ident master $b:block; $($tail:tt)*) => {
        $b;$crate::steer_build_m!( $name $($tail)*);
    };


    ( $name:ident send $event:ident; $($tail:tt)*) => {

        {
            let p = $event::new(&$name.metadata).await;
            let p = bincode::serialize(&p)?;
            let p = $event::encrypt(p, &$name.key.as_bytes()).await?;
            $name.stream.send(&p).await?;
        }
        $crate::steer_build_m!( $name $($tail)*);
    };
    ( $name:ident receive $event:ident; $($tail:tt)*) => {

        {
            let p = $name.stream.receive().await?;
            let p = $event::decrypt(p, &$name.key.as_bytes()).await?;
            let p = bincode::deserialize::<$event>(&p)?;
            p.respond(&$name.metadata).await;
        }
        $crate::steer_build_m!( $name $($tail)*);
    };

}

/// private macro used for the steer macro
#[macro_export(crate)]
macro_rules! steer_build_p {
    ($name:ident)=>{};
    ( $name:ident tx_handshake; $($tail:tt)*) => {
        $name.stream.receive_handshake().await?;
        $crate::steer_build_p!( $name $($tail)*);
    };
    ( $name:ident rx_handshake; $($tail:tt)*) => {
        $name.stream.send_handshake().await?;
        $crate::steer_build_p!( $name $($tail)*);
    };

    ( $name:ident tx_keepalive; $($tail:tt)*) => {
        $name.stream.receive_keepalive().await?;
        $crate::steer_build_p!( $name $($tail)*);
    };
    ( $name:ident rx_keepalive; $($tail:tt)*) => {
        $name.stream.send_keepalive().await?;
        $crate::steer_build_p!( $name $($tail)*);
    };

    ( $name: ident master $b:block; $($tail:tt)*) => {
        $crate::steer_build_m!( $name $($tail)*);
    };
    ( $name: ident both $b:block; $($tail:tt)*) => {
        $b;$crate::steer_build_m!( $name $($tail)*);
    };
    ( $name: ident peer $b:block; $($tail:tt)*) => {
        $b;$crate::steer_build_m!( $name $($tail)*);
    };

    ( $name:ident receive $event:ident; $($tail:tt)*) => {

        {
            let p = $event::new(&$name.metadata).await;
            let p = bincode::serialize(&p)?;
            let p = $event::encrypt(p, &$name.key.as_bytes()).await?;
            $name.stream.send(&p).await?;
        }
        $crate::steer_build_p!( $name $($tail)*);
    };
    ( $name:ident send $event:ident; $($tail:tt)*) => {

        {
            let p = $name.stream.receive().await?;
            let p = $event::decrypt(p, &$name.key.as_bytes()).await?;
            let p = bincode::deserialize::<$event>(&p)?;
            p.respond(&$name.metadata).await;
        }
        $crate::steer_build_p!( $name $($tail)*);
    };

}

/// deadlock-free ergonomic encrypted event passthrough
/// See README in repo for details.
/// available functions:
/// - tx_handshake        
///     transmit handshake
/// - rx_handshake        
///     receive handshake
/// - rx_keepalive        
///     receive keepalive
/// - tx_keepalive        
///     transmit keepalive
/// - send <Event>        
///     send event
/// - receive <Event>     
///     receive event
/// - peer {<block>}      
///     block of code to execute on peer
/// - master {<block>}    
///     block of code to execute on master
/// - both {<block>}      
///     block of code to execute on both
#[macro_export]
macro_rules! steer {
    () => {};
    (
        fn $master_func: ident ($type:ty) -> fn $peer_func: ident ($peer_type:ty),
        $($tail:tt)*
    ) => {

        #[async_trait::async_trait]
        trait MasterSteerTrait {
            async fn $master_func(&mut self) -> $crate::Res<()>;
        }

        #[async_trait::async_trait]
        trait PeerSteerTrait {
            async fn $peer_func(&mut self) -> $crate::Res<()>;
        }

        #[async_trait::async_trait]
        impl <T: tokio::io::AsyncWrite
        + tokio::io::AsyncRead
        + tokio::io::AsyncWriteExt
        + Unpin
        + Sync
        + Send> MasterSteerTrait for Steer<T, $type> {
            async fn $master_func(&mut self) -> $crate::Res<()> {
                $crate::steer_build_m!(self $($tail)*);
                Ok(())
            }
        }

        #[async_trait::async_trait]
        impl <T: tokio::io::AsyncWrite
        + tokio::io::AsyncRead
        + tokio::io::AsyncWriteExt
        + Unpin
        + Sync
        + Send> PeerSteerTrait for Steer<T, $peer_type> {
            async fn $peer_func(&mut self) -> $crate::Res<()> {
                $crate::steer_build_p!(self $($tail)*);
                Ok(())
            }
        }

        async fn $master_func<
            T: tokio::io::AsyncWrite
                + tokio::io::AsyncRead
                + tokio::io::AsyncWriteExt
                + Unpin
                + Sync
                + Send,
        >(
            mut st: Steer<T, $type>,
        ) -> $crate::Res<()> {
            $crate::steer_build_m!(st $($tail)*);
            Ok(())
        }
        async fn $peer_func<
            T: tokio::io::AsyncWrite
                + tokio::io::AsyncRead
                + tokio::io::AsyncWriteExt
                + Unpin
                + Sync
                + Send,
        >(
            mut st: Steer<T, $peer_type>,
        ) -> $crate::Res<()> {
            $crate::steer_build_p!(st $($tail)*);
            Ok(())
        }
    };
}

pub struct StdReactor<T> {
    pub metadata: T,
    pub services: Vec<tokio::task::JoinHandle<Res<()>>>,
}

#[async_trait]
impl<T: Sync + Send> Reactor for StdReactor<T> {
    type Metadata = T;
    type Services = Vec<tokio::task::JoinHandle<Res<()>>>;

    async fn init(metadata: Self::Metadata, services: Self::Services) -> Self {
        StdReactor { metadata, services }
    }
}

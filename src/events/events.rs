use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncRead, AsyncWrite};

use crate::encryption::Encrypt;

#[async_trait]
pub trait Event<'a>: Respond + Serialize + Deserialize<'a> + Encrypt {
    type SteerMeta;
    async fn new(input: &Self::SteerMeta) -> Self;
}

#[async_trait]
pub trait Respond {
    type SteerMetadata;
    type Output;
    async fn respond(self, metadata: &Self::SteerMetadata) -> Self::Output;
}

pub struct Steer<T: AsyncWrite + AsyncRead, X> {
    pub stream: T,
    pub key: String,
    pub metadata: X,
}

impl<T: AsyncWrite + AsyncRead, X> Steer<T, X> {
    pub async fn new(stream: T, key: String, metadata: X) -> Self {
        Self {
            stream,
            key,
            metadata,
        }
    }
}

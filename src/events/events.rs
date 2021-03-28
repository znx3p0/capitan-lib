
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::encryption::Encrypt;

#[async_trait]
pub trait Event<'a> : Respond + Serialize + Deserialize<'a> + Encrypt {
    type Meta;
    async fn new(input: Self::Meta) -> Self;
}

#[async_trait]
pub trait Respond {
    type Output;
    async fn respond(self) -> Self::Output;
}

use crate::Res;
use async_trait::async_trait;

#[async_trait]
pub trait Encrypt {
    async fn encrypt(input: Vec<u8>, key: &[u8]) -> Res<Vec<u8>>;
    async fn decrypt(out: Vec<u8>, key: &[u8]) -> Res<Vec<u8>>
    where
        Self: Sized;
}

use crate::Res;
use async_trait::async_trait;

#[async_trait]
pub trait Encrypt {
    async fn encrypt(input: Vec<u8>, key: &[u8]) -> Res<Vec<u8>> {
        Ok(input.into_iter().map(|s| s.rotate_right(s.count_ones() + *key.get(0).unwrap_or(&3) as u32)).collect())
    }
    async fn decrypt(out: Vec<u8>, key: &[u8]) -> Res<Vec<u8>> {
        Ok(out.into_iter().map(|s| s.rotate_left(s.count_ones() + *key.get(0).unwrap_or(&3) as u32)).collect())
    }
}

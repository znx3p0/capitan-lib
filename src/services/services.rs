
use async_trait::async_trait;

use crate::Res;

#[async_trait]
pub trait Service {
    type ReactorMetadata;

    /*
        methods are called the following way:
            init()
            loop {
                main()
                fallback()
            }
        there's more to it, but it's a simple representation
    */

    // this method will be called as before the main method
    async fn init(&self, input: Self::ReactorMetadata) -> Res<()>;
    // this method will be looped
    async fn main(&self, input: Self::ReactorMetadata) -> Res<()>;
    // this method will be called before looping the main method
    async fn fallback(&self, input: Self::ReactorMetadata) -> Res<()>;

}

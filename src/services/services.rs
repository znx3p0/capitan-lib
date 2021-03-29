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
    // if it returns an error, the service will abort
    async fn init(&self, input: &Self::ReactorMetadata) -> Res<()>;
    // this method will be looped
    // if it returns an error, the fallback will be run.
    async fn main(&self, input: &Self::ReactorMetadata) -> Res<()>;
    // this method will be called before looping the main method
    // ! if this method returns an error, the service will abort.
    async fn fallback(&self, input: &Self::ReactorMetadata) -> Res<()>;

    // this method will be called if the init service does not fail and the
    // fall back method fails.
    async fn abort(&self, input: &Self::ReactorMetadata) -> Res<()>;
}

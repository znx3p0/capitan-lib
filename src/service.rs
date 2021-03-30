
/*

THIS IS A SKETCH

macro_rules! service {
    (
        $name: ident
        $struct_vals: tt
        type ReactorMetadata = $meta: ty;

        $($t:tt)*
    ) => {
        struct $name $struct_vals
        #[async_trait::async_trait]
        impl $crate::services::Service for $name {
            type ReactorMetadata = $meta;
            service!($($t:tt)*);
        }
    };
    (
        async fn $fn_name:ident (&self, $ty_name: ident : &ReactorMetadata) $b:block
    ) => {
        async fn $fn_name
        (
            &self,
            $ty_name: Self::ReactorMetadata,
            spawner: &StdReactorServices<Self::ReactorMetadata>,
            service_id: &str
        ) -> $crate::Res<()> $b
    }
}


service! {

    HTTP {
        l: i32
    }
    type ReactorMetadata = ();
    async fn init(&self, input: &ReactorMetadata) {
        Ok(())
    }
    async fn main(&self, input: &ReactorMetadata) {

    }
    async fn fallback(&self, input: &ReactorMetadata) {

    }
    async fn abort(&self, input: &ReactorMetadata) {

    }
}


struct HTTP(u32);
#[async_trait]
impl Service for HTTP {
    type ReactorMetadata = ReactorMetadata;

    async fn init(&self, input: &Self::ReactorMetadata, spawner: &StdReactorServices<Self::ReactorMetadata>, service_id: &str) -> Res<()> {
        println!("initializing http");
        Ok(())
    }
    
    async fn main(&self, input: &Self::ReactorMetadata, spawner: &StdReactorServices<Self::ReactorMetadata>, service_id: &str) -> Res<()> {
        input.fetch_add(1, Ordering::Relaxed);
        println!("{:?}", input);
        Ok(())
    }
    
    async fn fallback(&self, input: &Self::ReactorMetadata, spawner: &StdReactorServices<Self::ReactorMetadata>, service_id: &str) -> Res<()> {
        println!("http fallback");
        Err(anyhow::anyhow!("asdf"))
    }

    async fn abort(&self, input: &Self::ReactorMetadata, spawner: &StdReactorServices<Self::ReactorMetadata>, service_id: &str) -> Res<()> {
        println!("aborting!");
        Ok(())
    }
}

should be equal to
service! {
    struct HTTP(u32);
    type ReactorMetadata = ReactorMetadata;
    
    async fn init(&self, input: &ReactorMetadata) {
        
    }
    async fn main(&self, input: &ReactorMetadata) {

    }
    async fn fallback(&self, input: &ReactorMetadata) {

    }
    async fn abort(&self, input: &ReactorMetadata) {

    }
}
*/


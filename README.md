# CAPITAN

## DISTRIBUTED SYSTEMS AND SERVICE LIBRARY (WIP)

## Service

```rust
// Services cannot
struct LB;
#[async_trait]
impl Service for LB {
    // the init method will only be called once
    // if it returns an error, the service will abort
    async fn init(&mut self) -> Res<()> {
        println!("initializing lb");
        Ok(())
    }
    
    // if the main method ends, the fallback will be called
    // and the main will follow.
    async fn main(&mut self) -> Res<()> {
        input.fetch_add(1, Ordering::Relaxed);
        println!("the reactor currently holds {:?}", input);
        Ok(())
    }

    // executes if main is successful
    async fn repeat(&mut self) -> Res<()> {
        Ok(())
    }
    
    // executes if main failed
    // if it returns Ok(()), the service will continue.
    async fn catch(&mut self, e: Err) -> Res<()> {
        println!("lb fallback");
        Ok(())
    }

    async fn abort(&mut self) -> Res<()> {
        println!("aborting");
        Ok(())
    }
}
```

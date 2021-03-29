# CAPITAN

## Distributed systems on a whim

I've worked in distributed systems for a while, and
most of my time has been spent writing boilerplate, debugging
little implementation errors and chasing down stream deadlocks
(two streams are receiving at the same time)
This library was built to combat the boilerplate that comes
with writing these kinds of applications and only write
meaningful code, in a way that is guaranteed to work and
avoid stream deadlocks.

This library provides macros and traits to help build
highly-available services with various features in as little
lines of code as possible.

This library is supposed to be protocol-agnostic, but is mostly
designed to work around the master/server architecture.

quick intro to Capitan

### SERVICES

```rust
// first off, there's services, which are highly available functions.
// they cannot crash unless the init or the fallback processes fail.
struct LB;
#[async_trait]
impl Service for LB {
    type ReactorMetadata = AtomicU32;

    // the init method will only be called once
    // if it returns an error, the service will abort
    async fn init(&self, input: &Self::ReactorMetadata) -> Res<()> {
        println!("initializing lb");
        Ok(())
    }
    
    // if the main method ends, the fallback will be called
    // and the main will follow.
    async fn main(&self, input: &Self::ReactorMetadata) -> Res<()> {
        input.fetch_add(1, Ordering::Relaxed);
        println!("the reactor currently holds {:?}", input);
        Ok(())
    }
    
    // the fallback method will be called after the main method
    // if it returns an error, the service will abort
    async fn fallback(&self, input: &Self::ReactorMetadata) -> Res<()> {
        println!("lb fallback");
        Ok(())
    }
}

// to initialize a service you create a reactor with it.
// this will run the LB service
capitan! {
    metadata: AtomicU32::new(0),
    services: [
        LB,
    ]
}
```

### EVENTS

```rust
type SteerMetadata = AtomicU32;

// events are message-based communication
// which are guaranteed not to have stream deadlocks
// they have to implement various traits to be able to be sent
// through the steer! macro
#[derive(Serialize, Deserialize)]
enum EventType {
    Available,
    Busy,
}

// this allows for easy encryption
// you can define the encryption algorithm.
#[async_trait]
impl Encrypt for EventType {
    async fn encrypt(input: Vec<u8>, _key: &[u8]) -> Res<Vec<u8>> {
        Ok(input)
    }
    
    async fn decrypt(out: Vec<u8>, _key: &[u8]) -> Res<Vec<u8>> {
        Ok(out)
    }
}


// the respond trait defines how to respond to each event.
// this trait must be implemented
#[async_trait]
impl Respond for EventType {
    type Output = ();
    type SteerMetadata = SteerMetadata;

    async fn respond(self, meta: &Self::SteerMetadata) -> Self::Output {
        match self {
            EventType::Available => {
                println!("alive")
            },
            EventType::Dead => {
                println!("dead")
            }
        };
        ()
    }
}

// The event trait defines which event to create.
#[async_trait]
impl Event<'_> for EventType {
    type SteerMeta = SteerMetadata;

    async fn new(input: &SteerMetadata) -> Self {
        if input.load(Ordering::Relaxed) % 2 == 0 {
            EventType::Available
        } else {
            EventType::Busy
        }
    }
}

// having once implemented all traits, you can now use
// the steer macro for simple, safe, deadlock-guaranteed
// message passthrough.
steer! {
    fn master_event(Arc<AtomicU32>) -> fn peer_event(Arc<AtomicU32>),
    rx_handshake;
    receive EventType;
    tx_keepalive;
}

// this will create two async methods for the steer struct
// async fn master_event()
// and
// async fn peer_event()
let stream = TcpStream::connect("127.0.0.1:28343").await?;
let mut steer = Steer::new(stream, "encryption_key".to_string(), Arc::new(AtomicU32::new(0))).await;
steer.master_event().await?;

// on the peer you run
// steer.peer_event().await?;
```

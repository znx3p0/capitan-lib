use anyhow::Error;

pub mod encryption;
pub mod events;
pub mod macros;
pub mod serialization;
pub mod services;
pub mod structs;
pub mod reactor;
pub mod capitan;

pub type Res<T> = Result<T, Error>;

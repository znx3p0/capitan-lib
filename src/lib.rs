use anyhow::Error;

pub mod capitan;
pub mod service;
pub mod encryption;
pub mod events;
pub mod macros;
pub mod reactor;
pub mod serialization;
pub mod services;
pub mod stream;
pub mod structs;

pub type Res<T> = Result<T, Error>;

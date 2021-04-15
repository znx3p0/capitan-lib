mod dynamic;
mod isolated;
mod macros;
mod reactor;
mod shared;

pub use isolated::IsolatedService;
pub use shared::SharedService;

pub use reactor::{IsolatedReactor, SharedReactor};

pub mod prelude {
    pub use super::reactor::{IsolatedReactorTrait as IsolatedR, SharedReactorTrait as SharedR};
    pub use super::{isolated::IsolatedService, shared::SharedService};
}

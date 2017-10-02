use std::any::Any;

mod actor;
mod actor_ref;
mod actor_system;
mod actor_factory;
mod actor_context;
mod actor_path;
mod props;

pub use actor::*;
pub use actor_ref::*;
pub use actor_system::*;
pub use actor_factory::*;
pub use actor_context::*;
pub use actor_path::*;
pub use props::*;

pub mod prelude {
    pub use {Actor, ActorContext, ActorFactory, ActorPath, ActorRef, ActorSystem, Props};
}

pub trait Message: Any + Clone + Send + Sync + 'static {}

impl<A> Message for A
where
    A: Any + Clone + Send + Sync + 'static,
{
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}

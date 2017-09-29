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
    pub use {ActorSystem, Actor, ActorRef, Props, ActorFactory, ActorContext, ActorPath};
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

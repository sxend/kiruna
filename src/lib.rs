mod actor;
mod actor_ref;
mod actor_system;
mod actor_factory;
mod props;
pub use actor::*;
pub use actor_ref::*;
pub use actor_system::*;
pub use actor_factory::*;
pub use props::*;

pub mod prelude {
    pub use {ActorSystem, Actor, Props, ActorFactory};
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

mod actor;

pub use actor::*;

pub mod prelude {
    pub use {Actor, ActorContext, ActorFactory, ActorPath, ActorRef, ActorSystem, Props};
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}

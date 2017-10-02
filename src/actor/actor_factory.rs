use std::sync::Arc;

use actor::{Actor, Props};

pub trait ActorFactory: Send + Sync {
    fn create(&self) -> Arc<Actor>;
}

impl<A: Actor> ActorFactory for Props<A> {
    fn create(&self) -> Arc<Actor> {
        self.new_actor()
    }
}

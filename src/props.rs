use std::sync::Arc;
use actor::Actor;
use actor_factory::ActorFactory;

pub struct Props<A: Actor> {
    factory: Arc<Fn() -> A + Sync + Send>
}

impl<A: Actor> Props<A> {

    pub fn new(factory: Arc<Fn() -> A + Sync + Send>) -> Arc<ActorFactory> {
        Arc::new(Props {
            factory: factory,
        })
    }
    pub fn new_actor(&self) -> Arc<Actor> {
        let f = self.factory.clone();
        Arc::new(f())
    }
}
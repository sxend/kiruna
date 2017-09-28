
//pub type String = Send + Sync;
use std::any::Any;
use std::sync::Arc;

pub struct ActorSystem {
    name: String
}

impl ActorSystem {
    pub fn new(name: &str) -> ActorSystem {
        ActorSystem {
            name: name.to_string()
        }
    }
    pub fn actor_of(&self, props: Arc<ActorFactory>) -> ActorRef {
        ActorRef {
            inner: props.clone().create()
        }
    }
}

pub struct ActorRef {
    inner: Arc<Actor>,
}

impl ActorRef {
    pub fn send(&self, message: Box<Any>) {
        self.inner.receive(message)
    }
}

pub trait Actor: Send + Sync + 'static {
    fn receive(&self, message: Box<Any>);
}

pub struct Props<A: Actor> {
    factory: Arc<Fn() -> A + Sync + Send>
}


impl<A: Actor> Props<A> {

    pub fn new(factory: Arc<Fn() -> A + Sync + Send>) -> Arc<ActorFactory> {
        Arc::new(Props {
            factory: factory,
        })
    }
    fn new_actor(&self) -> Arc<Actor> {
        let f = self.factory.clone();
        Arc::new(f())
    }
}
pub trait ActorFactory: Send + Sync {
    fn create(&self) -> Arc<Actor>;
}

impl<A: Actor> ActorFactory for Props<A> {
    fn create(&self) -> Arc<Actor> {
        self.new_actor()
    }
}

pub mod prelude {
    pub use {ActorSystem, Actor, Props, ActorFactory};
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

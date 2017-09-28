
//pub type String = Send + Sync;

pub struct ActorSystem {

}

impl ActorSystem {
    pub fn new(name: &str) -> ActorSystem {
        ActorSystem {}
    }
    pub fn actor_of<A: 'static + Actor, F: Fn() -> A>(&self, props: Props<A, F>) -> Box<ActorRef> {
        let local = LocalActorRef::new(props.new_actor());
        Box::new(local)
    }
}

pub trait ActorRef {
    fn send(&self, message: String);
}

struct LocalActorRef<A: Actor> {
    internal: A
}
impl<A: Actor> LocalActorRef<A> {
    fn new(internal: A) -> LocalActorRef<A> {
        LocalActorRef {
            internal: internal
        }
    }
}
impl<'a, A: Actor> ActorRef for LocalActorRef<A> {
    fn send(&self, message: String) {
        self.internal.receive(message);
    }
}

pub trait Actor {
    fn receive(&self, message: String);
}

pub struct Props<A: Actor, F: Fn() -> A> {
    factory: F,
}

impl<A: Actor, F: Fn() -> A> Props<A, F> {

    pub fn new(factory: F) -> Props<A, F> {
        Props {
            factory: factory,
        }
    }
    fn new_actor(&self) -> A {
        let factory = &self.factory;
        factory()
    }
}

trait ActorFactory<A: Actor> {
    fn create(&self) -> A;
}

pub mod prelude {
    pub use {ActorSystem, Actor, Props};
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

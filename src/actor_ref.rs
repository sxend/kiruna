use std::sync::Arc;
use std::any::Any;
use actor::Actor;

pub struct ActorRef {
    inner: Arc<Actor>,
}

impl ActorRef {
    pub fn new(inner: Arc<Actor>) -> ActorRef {
        ActorRef {
            inner: inner
        }
    }
    pub fn send(&self, message: Box<Any>) {
        self.inner.receive(message)
    }
}

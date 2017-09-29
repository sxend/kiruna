use std::any::Any;
use std::sync::Arc;
use actor_ref::ActorRef;
use actor_context::ActorContext;

pub trait Actor: Send + Sync + 'static {
    fn receive(&self, context: Arc<ActorContext>, message: Box<Any>);
}
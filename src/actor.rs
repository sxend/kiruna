use std::any::Any;
use std::sync::Arc;
use std::sync::mpsc::*;
use actor_context::ActorContext;

pub trait Actor: Send + Sync + 'static {
    fn receive(&self, sender: Sender<Box<Any + Send + Sync>>, context: Arc<ActorContext>, message: Box<Any>);
}
use std::any::Any;
use std::sync::Arc;
use std::sync::mpsc::*;

mod actor_context;
mod actor_system;
mod actor_ref;
mod actor_factory;
mod props;
mod actor_path;


pub use self::actor_ref::*;
pub use self::actor_system::*;
pub use self::actor_factory::*;
pub use self::actor_context::*;
pub use self::actor_path::*;
pub use self::props::*;


pub trait Actor: Send + Sync + 'static {
    fn receive(
        &self,
        sender: Sender<Box<Any + Send + Sync>>,
        context: Arc<ActorContext>,
        message: Box<Any>,
    );
}

pub trait Message: Any + Clone + Send + Sync + 'static {}

impl<A> Message for A
    where
        A: Any + Clone + Send + Sync + 'static,
{
}

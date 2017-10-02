
use actor::{InnerActorRef, ActorRef};
use actor_system::ActorSystem;

pub struct Envelope {
    message: Box<Any + Send + Sync>,
    sender: ActorRef,
    system: ActorSystem,
}

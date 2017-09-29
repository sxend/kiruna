extern crate kiruna;

use kiruna::prelude::*;
use std::sync::Arc;
use std::any::Any;
use std::fmt::Display;

fn main() {
    let actor_system = ActorSystem::new("name");
    let props = Props::new(Arc::new(|| SampleActor));
    let actor_ref = actor_system.actor_of(props);
    actor_ref.send(Box::new(SampleMessage("hello".to_string())));
}

struct SampleActor;

impl Actor for SampleActor {
    fn receive(&self, msg: Box<Any>) {
        if let Ok(message) = Box::<Any>::downcast::<SampleMessage>(msg) {
            println!("{}", *message);
        }
    }
}

#[derive(Debug)]
struct SampleMessage(String);
unsafe impl Send for SampleMessage {}
unsafe impl Sync for SampleMessage {}
impl Display for SampleMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}
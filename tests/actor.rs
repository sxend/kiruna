extern crate kiruna;

use std::sync::Arc;
use std::any::Any;
use kiruna::prelude::*;

#[test]
fn actor_system() {
    let actor_system = ActorSystem::new("name".to_string());
    let actor_ref = actor_system.actor_of(Props::new(Arc::new(|| SampleActor)));
    actor_ref.send(Box::new("hello".to_string()));
}

struct SampleActor;

impl Actor for SampleActor {
    fn receive(&self, sender: Arc<ActorRef>, context: Arc<ActorContext>, msg: Box<Any>) {
        println!("{:?}", msg);
    }
}
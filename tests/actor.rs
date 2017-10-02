extern crate kiruna;

use std::sync::Arc;
use std::any::Any;
use std::sync::mpsc::Sender;
use kiruna::prelude::*;

#[test]
fn actor_system() {
    let actor_system = ActorSystem::new("name".to_string());
    let actor_ref = actor_system.actor_of(Props::new(|| SampleActor), "sample".to_owned());
    actor_ref.send("hello".to_string());
}

struct SampleActor;

impl Actor for SampleActor {
    fn receive(
        &self,
        sender: Sender<Box<Any + Send + Sync>>,
        context: Arc<ActorContext>,
        msg: Box<Any>,
    ) {
        println!("{:?}", msg);
    }
}

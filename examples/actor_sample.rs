extern crate kiruna;

use kiruna::prelude::*;
use std::sync::Arc;
use std::any::Any;
use std::fmt::{Debug, Display};
use std::sync::mpsc::*;

fn main() {
    let actor_system = ActorSystem::new("name".to_string());
    let props = Props::new(|| SampleActor);
    let actor_ref = actor_system.actor_of(props, "sample".to_owned());
    actor_ref.send(SampleMessage("hello".to_string()));
    let result = actor_ref
        .ask(SampleMessage("hello".to_string()))
        .recv()
        .unwrap();
    if let Ok(result) = Box::<Any>::downcast::<String>(result) {
        println!("{}", result);
    }
    let (tx, rx): (Sender<String>, Receiver<String>) = channel();
    rx.recv().unwrap();
}

struct SampleActor;

impl Actor for SampleActor {
    fn receive(
        &self,
        sender: Sender<Box<Any + Send + Sync>>,
        context: Arc<ActorContext>,
        msg: Box<Any>,
    ) {
        if let Ok(message) = Box::<Any>::downcast::<SampleMessage>(msg) {
            println!("{}", *message);
            let _ = sender.send(Box::new("finish".to_string()));
        }
    }
}

#[derive(Debug, Clone)]
struct SampleMessage(String);
unsafe impl Send for SampleMessage {}
unsafe impl Sync for SampleMessage {}
impl Display for SampleMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

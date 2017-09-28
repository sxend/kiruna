extern crate kiruna;

use kiruna::prelude::*;

#[test]
fn actor_system() {
    let actor_system = ActorSystem::new("name");
    let actor_ref = actor_system.actor_of(Props::new(|| SampleActor));
    actor_ref.send("hello".to_string());
}

struct SampleActor;

impl Actor for SampleActor {
    fn receive(&self, msg: String) {
        println!("{:?}", msg);
    }
}
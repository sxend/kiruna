use std::sync::Arc;
use actor_ref::ActorRef;
use actor_factory::ActorFactory;

pub struct ActorSystem {
    name: String
}

impl ActorSystem {
    pub fn new(name: String) -> ActorSystem {
        ActorSystem {
            name: name
        }
    }
    pub fn actor_of(&self, props: Arc<ActorFactory>) -> ActorRef {
        ActorRef::new(props.clone().create())
    }
    pub fn get_name(&self) -> String {
        self.name.to_owned()
    }
}
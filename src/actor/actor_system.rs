extern crate jobpool;

use std::sync::Arc;
use std::sync::Mutex;
use actor::{ActorRef, ActorFactory};
use self::jobpool::JobPool;

pub struct ActorSystem {
    name: String,
    inner: Arc<InnerActorSystem>,
}

impl ActorSystem {
    pub fn new(name: String) -> ActorSystem {
        ActorSystem {
            name: name.to_owned(),
            inner: Arc::new(InnerActorSystem::new(name.to_owned())),
        }
    }
    pub fn actor_of(&self, props: Arc<ActorFactory>, name: String) -> ActorRef {
        self.inner.actor_of(props, name)
    }
}

struct InnerActorSystem {
    name: String,
    dispatcher: Arc<Mutex<JobPool>>,
}

impl InnerActorSystem {
    fn new(name: String) -> InnerActorSystem {
        InnerActorSystem {
            name,
            dispatcher: Arc::new(Mutex::new(JobPool::new(4))),
        }
    }
    pub fn actor_of(&self, props: Arc<ActorFactory>, name: String) -> ActorRef {
        let actor_ref = ActorRef::new(props.clone().create(), self.dispatcher.clone());
        actor_ref
    }
}

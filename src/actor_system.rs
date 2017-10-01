extern crate jobpool;

use std::sync::Arc;
use std::sync::Mutex;
use actor_ref::ActorRef;
use actor_path::ActorPath;
use actor_factory::ActorFactory;
use self::jobpool::JobPool;

pub struct ActorSystem {
    name: String,
    actor_refs: Arc<Mutex<Vec<Arc<ActorRef>>>>,
    pool: Arc<Mutex<JobPool>>
}

impl ActorSystem {
    pub fn new(name: String) -> ActorSystem {
        ActorSystem {
            name: name,
            actor_refs: Arc::new(Mutex::new(vec![])),
            pool: Arc::new(Mutex::new(JobPool::new(4)))
        }
    }
    pub fn actor_of(&self, props: Arc<ActorFactory>) -> Arc<ActorRef> {
        let actor_ref = Arc::new(ActorRef::new(props.clone().create(), self.pool.clone()));
        let actor_refs = self.actor_refs.clone();
        let actor_refs = actor_refs.lock();
        let mut actor_refs = actor_refs.unwrap();
        actor_refs.push(actor_ref.clone());
        actor_ref
    }
    pub fn get_name(&self) -> String {
        self.name.to_owned()
    }
}
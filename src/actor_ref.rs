extern crate jobpool;

use std::sync::Arc;
use std::any::Any;
use std::sync::Mutex;
use actor::Actor;
use actor_context::ActorContext;
use self::jobpool::JobPool;

pub struct ActorRef {
    inner: InnerActorRef,
    pool: Arc<JobPool>,
}

impl ActorRef {
    pub fn new(underlying: Arc<Actor>, pool: Arc<JobPool>) -> ActorRef {
        ActorRef {
            inner: InnerActorRef {
                underlying: underlying,
                mailbox: Arc::new(Mutex::new(vec![])),
            },
            pool: pool
        }
    }
    pub fn send(&self, message: Box<Any>) {
        let mailbox = self.inner.mailbox.clone();
        let mut mailbox = mailbox.lock();
        println!("push message");
        mailbox.unwrap().push(message);
    }
}

struct InnerActorRef {
    underlying: Arc<Actor>,
    mailbox: Arc<Mutex<Vec<Box<Any>>>>
}
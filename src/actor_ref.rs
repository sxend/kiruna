extern crate jobpool;

use std::sync::Arc;
use std::any::Any;
use std::sync::Mutex;
use actor::Actor;
use actor_context::ActorContext;
use self::jobpool::JobPool;

pub struct ActorRef {
    inner: Arc<InnerActorRef>,
    pool: Arc<JobPool>,
}

impl ActorRef {
    pub fn new(underlying: Arc<Actor>, pool: Arc<JobPool>) -> ActorRef {
        ActorRef {
            inner: Arc::new(InnerActorRef {
                underlying: underlying,
                mailbox: Arc::new(Mutex::new(vec![])),
            }),
            pool: pool
        }
    }
    pub fn send(&self, message: Box<Any + Send + Sync>) {
        let mailbox = self.inner.mailbox.clone();
        let mut mailbox = mailbox.lock();
        mailbox.unwrap().push(message);
        let mut pool = self.pool.clone();
        let inner = self.inner.clone();
        pool.queue(move || {
            let mailbox = inner.mailbox.clone();
            let message = mailbox.lock().unwrap().pop().unwrap();
            let underlying = inner.underlying.clone();
            underlying.receive(Arc::new(ActorContext), message);
        });
    }
}

struct InnerActorRef {
    underlying: Arc<Actor>,
    mailbox: Arc<Mutex<Vec<Box<Any + Send + Sync>>>>
}
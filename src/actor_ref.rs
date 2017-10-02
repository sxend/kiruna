extern crate jobpool;

use std::sync::Arc;
use std::any::Any;
use std::sync::Mutex;
use std::sync::mpsc::*;
use actor::Actor;
use actor_context::ActorContext;
use Message;
use self::jobpool::JobPool;

pub struct ActorRef {
    inner: Arc<InnerActorRef>,
    pool: Arc<Mutex<JobPool>>,
}

impl ActorRef {
    pub fn new(underlying: Arc<Actor>, pool: Arc<Mutex<JobPool>>) -> ActorRef {
        ActorRef {
            inner: Arc::new(InnerActorRef {
                underlying: underlying,
                mailbox: Arc::new(Mutex::new(vec![])),
            }),
            pool
        }
    }
    pub fn send<M: Message>(&self, message: M) {
        self.ask(message);
    }
    pub fn ask<M: Message>(&self, message: M) -> Receiver<Box<Any + Send + Sync>> {
        let mailbox = self.inner.mailbox.clone();
        let mailbox = mailbox.lock();
        mailbox.unwrap().push(Box::new(message));
        let inner = self.inner.clone();
        let (tx, rx): (Sender<Box<Any + Send + Sync>>, Receiver<Box<Any + Send + Sync>>) = channel();
        let tx = tx.clone();
        let pool = self.pool.clone();
        let pool = pool.lock();
        pool.unwrap().queue(move || {
            let mailbox = inner.mailbox.clone();
            let message = mailbox.lock().unwrap().pop().unwrap();
            let underlying = inner.underlying.clone();
            underlying.receive(tx, Arc::new(ActorContext), message);
        });
        rx
    }
}

struct InnerActorRef {
    underlying: Arc<Actor>,
    mailbox: Arc<Mutex<Vec<Box<Any + Send + Sync>>>>
}
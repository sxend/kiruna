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
}

impl ActorRef {
    pub fn new(underlying: Arc<Actor>, dispatcher: Arc<Mutex<JobPool>>) -> ActorRef {
        ActorRef {
            inner: Arc::new(InnerActorRef::new(underlying, dispatcher)),
        }
    }
    pub fn send<M: Message>(&self, message: M) {
        self.inner.send(message)
    }
    pub fn ask<M: Message>(&self, message: M) -> Receiver<Box<Any + Send + Sync>> {
        self.inner.ask(message)
    }
}

struct InnerActorRef {
    underlying: Arc<Actor>,
    mailbox: Arc<Mutex<Vec<(Box<Any + Send + Sync>, Sender<Box<Any + Send + Sync>>)>>>,
    dispatcher: Arc<Mutex<JobPool>>,
}
impl InnerActorRef {
    fn new(underlying: Arc<Actor>,
           dispatcher: Arc<Mutex<JobPool>>) -> InnerActorRef {
        InnerActorRef {
            underlying: underlying,
            mailbox: Arc::new(Mutex::new(vec![])),
            dispatcher: dispatcher
        }
    }
    pub fn send<M: Message>(&self, message: M) {
        self.ask(message);
    }
    pub fn ask<M: Message>(&self, message: M) -> Receiver<Box<Any + Send + Sync>> {
        let mailbox = self.mailbox.clone();
        let mailbox = mailbox.lock();
        let (tx, rx): (Sender<Box<Any + Send + Sync>>, Receiver<Box<Any + Send + Sync>>) = channel();
        mailbox.unwrap().push((Box::new(message), tx.clone()));

        let dispatcher = self.dispatcher.clone();
        let dispatcher = dispatcher.lock();
        let mailbox = self.mailbox.clone();
        let underlying = self.underlying.clone();
        dispatcher.unwrap().queue(move || {
            let (message, tx) = mailbox.lock().unwrap().pop().unwrap();
            underlying.receive(tx, Arc::new(ActorContext), message);
        });
        rx
    }
}

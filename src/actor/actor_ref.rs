extern crate jobpool;

use std::sync::Arc;
use std::any::Any;
use std::sync::Mutex;
use std::sync::mpsc::*;
use actor::{Actor, ActorContext, Message};
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

pub struct InnerActorRef {
    underlying: Arc<Actor>,
    mailbox: Arc<Mutex<Vec<(Box<Any + Send + Sync>, Sender<Box<Any + Send + Sync>>)>>>,
    dispatcher: Arc<Mutex<JobPool>>,
}
impl InnerActorRef {
    fn new(underlying: Arc<Actor>, dispatcher: Arc<Mutex<JobPool>>) -> InnerActorRef {
        let inner = InnerActorRef {
            underlying: underlying,
            mailbox: Arc::new(Mutex::new(vec![])),
            dispatcher: dispatcher,
        };
        InnerActorRef::start_loop(
            inner.underlying.clone(),
            inner.mailbox.clone(),
            inner.dispatcher.clone(),
        );
        inner
    }
    fn start_loop(
        underlying: Arc<Actor>,
        mailbox: Arc<Mutex<Vec<(Box<Any + Send + Sync>, Sender<Box<Any + Send + Sync>>)>>>,
        dispatcher: Arc<Mutex<JobPool>>,
    ) {
        let _dispatcher = dispatcher.clone();
        let _dispatcher = _dispatcher.lock();
        let mut _dispatcher = _dispatcher.unwrap();
        _dispatcher.queue(move || {
            let len = mailbox.lock().unwrap().len();
            for _ in 0..len {
                // TODO max execution
                let (message, tx) = mailbox.lock().unwrap().pop().unwrap();
                underlying.receive(tx, Arc::new(ActorContext {}), message); // TODO error handling
            }
            InnerActorRef::start_loop(underlying.clone(), mailbox.clone(), dispatcher.clone());
        });
    }
    pub fn send<M: Message>(&self, message: M) {
        self.ask(message);
    }
    pub fn ask<M: Message>(&self, message: M) -> Receiver<Box<Any + Send + Sync>> {
        let mailbox = self.mailbox.clone();
        let mailbox = mailbox.lock();
        let (tx, rx): (
            Sender<Box<Any + Send + Sync>>,
            Receiver<Box<Any + Send + Sync>>,
        ) = channel();
        mailbox.unwrap().push((Box::new(message), tx.clone()));
        rx
    }
}

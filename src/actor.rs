use std::any::Any;

pub trait Actor: Send + Sync + 'static {
    fn receive(&self, message: Box<Any>);
}
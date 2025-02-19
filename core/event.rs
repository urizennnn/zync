use std::sync::mpsc::{channel, Receiver, Sender};

#[derive(Debug)]
pub struct SyncEvent<T> {
    sender: Sender<T>,
    receiver: Receiver<T>,
}

pub trait SyncTrait<T> {
    fn new() -> Self;
    fn clone_sender(&self) -> Sender<T>;
    fn send(&self, data: T);
    fn recv(&mut self) -> Option<T>;
}

impl<T> SyncTrait<T> for SyncEvent<T>
where
    T: Send + 'static,
{
    fn new() -> Self {
        let (sender, receiver) = channel();
        Self { sender, receiver }
    }

    fn send(&self, data: T) {
        let _ = self.sender.send(data);
    }

    fn recv(&mut self) -> Option<T> {
        self.receiver.recv().ok()
    }

    fn clone_sender(&self) -> Sender<T> {
        self.sender.clone()
    }
}

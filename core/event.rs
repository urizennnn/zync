use tokio::sync::mpsc::{channel, Receiver, Sender};

pub struct AsyncEvent<T> {
    sender: Sender<T>,
    receiver: Receiver<T>,
}

pub trait NewTrait<T> {
    fn new() -> Self;

    fn send(&self, data: T) -> impl std::future::Future<Output = ()> + Send;

    fn recv(&mut self) -> impl std::future::Future<Output = Option<T>>;
}

impl<T> NewTrait<T> for AsyncEvent<T>
where
    T: Send + 'static,
{
    fn new() -> Self {
        let (sender, receiver) = channel(32);
        Self { sender, receiver }
    }

    fn send(&self, data: T) -> impl std::future::Future<Output = ()> + Send {
        let sender = self.sender.clone();
        async move {
            if let Err(err) = sender.send(data).await {
                eprintln!("Failed to send data: {}", err);
            }
        }
    }

    fn recv(&mut self) -> impl std::future::Future<Output = Option<T>> {
        self.receiver.recv()
    }
}

use tokio::sync::mpsc::{channel, Receiver, Sender};

pub struct AsyncEvent<T> {
    sender: Sender<T>,
    receiver: Receiver<T>,
}

pub trait NewTrait<T> {
    fn new(buffer_size: usize) -> Self;

    fn send(&self, data: T) -> impl std::future::Future<Output = ()> + Send;

    fn recv(&mut self) -> impl std::future::Future<Output = Option<T>>;
}

impl<T> NewTrait<T> for AsyncEvent<T>
where
    T: Send + 'static,
{
    fn new(buffer_size: usize) -> Self {
        let (sender, receiver) = channel(buffer_size);
        Self { sender, receiver }
    }

    async fn send(&self, data: T) {
        if let Err(err) = self.sender.send(data).await {
            eprintln!("Failed to send data: {}", err);
        }
    }

    async fn recv(&mut self) -> Option<T> {
        self.receiver.recv().await
    }
}

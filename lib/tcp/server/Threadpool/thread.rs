use core::fmt;
use log::{info, warn};
use std::{
    error::Error,
    fmt::Debug,
    sync::{
        Arc, Mutex,
        mpsc::{self, Receiver, Sender},
    },
    thread,
};
use tokio::runtime::Runtime;

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct Threadpool {
    threads: Vec<Worker>,
    sender: Sender<Message>,
}

pub struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

#[derive(Debug)]
pub struct PoolCreationError;

impl fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Size must be greater than zero")
    }
}

impl Error for PoolCreationError {}

impl Threadpool {
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }

    pub fn build(size: usize) -> Result<Threadpool, PoolCreationError> {
        if size == 0 {
            return Err(PoolCreationError);
        }

        let (tx, rx) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(rx));
        let mut threads = Vec::with_capacity(size);

        for id in 0..size {
            threads.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Ok(Threadpool {
            threads,
            sender: tx,
        })
    }
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || {
            let rt = Runtime::new().expect("Failed to create Tokio runtime");

            loop {
                let message = receiver.lock().unwrap().recv();

                match message {
                    Ok(Message::NewJob(job)) => {
                        info!("Worker {id} got a job; executing.");

                        // Execute the job within the Tokio runtime
                        rt.block_on(async {
                            job();
                        });
                    }
                    Ok(Message::Terminate) => {
                        info!("Worker {id} was told to terminate.");
                        break;
                    }
                    Err(_) => {
                        info!("Worker {id} disconnected; shutting down.");
                        break;
                    }
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

impl Drop for Threadpool {
    fn drop(&mut self) {
        warn!("Sending termination signal to all workers.");

        for _ in &mut self.threads {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.threads {
            warn!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

enum Message {
    NewJob(Job),
    Terminate,
}

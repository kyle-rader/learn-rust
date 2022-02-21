use core::fmt;
use std::{
    sync::{mpsc, Arc, Mutex},
    thread::{self, JoinHandle},
};

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

#[derive(Debug)]
pub struct ThreadPoolCreationError(String);

impl fmt::Display for ThreadPoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct ThreadPoolWorker {
    id: usize,
    thread: Option<JoinHandle<()>>,
}

impl ThreadPoolWorker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> ThreadPoolWorker {
        let thread = Some(thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::NewJob(job) => {
                    println!("Worker {id} got a message. Executing.");
                    job();
                }
                Message::Terminate => {
                    println!("Worker {id} terminating.");
                    break;
                }
            }
        }));

        ThreadPoolWorker { id, thread }
    }
}

pub struct ThreadPool {
    workers: Vec<ThreadPoolWorker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    /// Create a new ThreadPool
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> Result<ThreadPool, ThreadPoolCreationError> {
        match size {
            0 => Err(ThreadPoolCreationError(String::from(
                "Pool size must be at least 1!",
            ))),
            _ => Ok({
                let (sender, receiver) = mpsc::channel();
                let receiver = Arc::new(Mutex::new(receiver));

                let mut workers = Vec::with_capacity(size);
                for id in 0..size {
                    workers.push(ThreadPoolWorker::new(id, Arc::clone(&receiver)));
                }

                ThreadPool { workers, sender }
            }),
        }
    }

    pub fn execute<F>(&mut self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending Terminate to all workers...");
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

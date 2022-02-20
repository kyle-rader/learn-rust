use core::fmt;
use std::{
    sync::{mpsc, Arc, Mutex},
    thread::{self, JoinHandle},
};

#[derive(Debug)]
pub struct ThreadPoolCreationError(String);

impl fmt::Display for ThreadPoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct ThreadPoolWorker {
    id: usize,
    thread: JoinHandle<()>,
}

impl ThreadPoolWorker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> ThreadPoolWorker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {id} got a job. Executing.");
            job();
        });

        ThreadPoolWorker { id, thread }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<ThreadPoolWorker>,
    sender: mpsc::Sender<Job>,
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
        self.sender.send(job).unwrap();
    }
}

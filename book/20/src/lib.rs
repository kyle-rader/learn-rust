use core::fmt;
use std::thread;

#[derive(Debug)]
pub struct ThreadPoolCreationError(String);

impl fmt::Display for ThreadPoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
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
                let mut threads = Vec::with_capacity(size);
                for _ in 0..size {
                    // create some threads and store them in the vector
                }
                ThreadPool { threads }
            }),
        }
    }

    pub fn execute<F>(f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}

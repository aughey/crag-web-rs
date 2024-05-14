use std::error;
use std::fmt;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
pub enum PoolCreationError {
    ZeroSize,
}

impl fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::Error for PoolCreationError {}

#[derive(Debug)]
pub struct ThreadPool {
    _workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    /// Create a new ThreadPool
    ///
    /// The size is the number of threads in the pool.
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size < 1 {
            return Err(PoolCreationError::ZeroSize);
        }

        // use mpsc channel to send requests to workers
        let (sender, receiver) = mpsc::channel();

        // workers will need a Mutex to share the receiver
        // across threads so we wrap in thread safe
        // smart pointer Arc
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // give each worker a clone of the Arc that holds the receiver
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Ok(ThreadPool {
            _workers: workers,
            sender,
        })
    }
    /// Execute a request in the stream by being passed in the
    /// handle_connection function as a closure
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job: Job = Box::new(f);
        self.sender.send(job).expect("sender error");
    }
}

#[derive(Debug)]
pub struct Worker {
    _id: usize,
    _thread: thread::JoinHandle<()>,
}

impl Worker {
    /// Create a new Worker with a receiver clone
    /// and spawns a thread that loops over jobs sent over the
    /// receiver and executes the job
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // if OS cannot create a new thread, thread::spawn will panic
        // TODO: Change to thread::Builder which returns Result
        let thread = thread::spawn(move || loop {
            // blocks all other threads trying to aquire lock
            // until it goes out of scope
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {id} received job. Executing.");

            job();
        });

        Worker {
            _id: id,
            _thread: thread,
        }
    }
}

/// Type alias for the closure arument to ThreadPool.execute()
type Job = Box<dyn FnOnce() + Send + 'static>;

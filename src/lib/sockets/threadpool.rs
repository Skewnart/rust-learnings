use std::{sync::{mpsc, Arc, Mutex}, thread::{self, JoinHandle}};
use crate::lib::sockets::pool_creation_error::PoolCreationError;

struct Worker {
    id: usize,
    thread: JoinHandle<()>
}

impl Worker {
    fn new(id : usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv();

                match message {
                    Ok(job) => {
                        println!("Worker {id} got a job. Executing...");
                        job();
                    }
                    Err(_) => {
                        println!("Worker {id} is disconnected. Shutting down...");
                        break;
                    }
                }
            }
        });

        Self { id, thread }
    }
}

pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>
}
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    fn new(size : usize) -> Self {
        let mut workers = Vec::with_capacity(size);
        let (sender, receiver) = mpsc::channel();
        
        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Self { workers, sender: Some(sender) }
    }

    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size == 0 { return Err (PoolCreationError { }); }

        Ok(Self::new(size))
    }

    pub fn execute<F>(&self, f: F)
        where F : FnOnce() -> (),
            F : Send + 'static {
                let job = Box::new(f);
                self.sender.as_ref().unwrap().send(job).unwrap();
        }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in self.workers.drain(..) {
            println!("Shutting down worker {}", worker.id);
            worker.thread.join().unwrap();
        }
    }
}

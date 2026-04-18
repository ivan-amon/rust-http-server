use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
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
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        // We wrap the receiver in an Arc<Mutex<T>> to allow shared ownership
        // among multiple workers, solving the "single consumer" problem.
        //
        // 1. Arc: Enables multiple threads to own the receiver (overcoming
        //    mpsc's "Single Consumer" ownership restriction).
        // 2. Mutex: Ensures only one worker at a time can access the receiver
        //    to pull a job, preventing data races during mutation.
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Doesn't panic
    ///
    /// The `build` function returns Result
    pub fn build(size: usize) -> Result<Self, ThreadPoolError> {
        if size == 0 {
            return Err(ThreadPoolError::SizeZero);
        }
        Ok(ThreadPool::new(size))
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
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

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || {
            // Passive waiting, .recv() is a blocking task
            // .recv() over .try_recv() to avoid spinning
            loop {
                let msg = receiver.lock().unwrap().recv();
                match msg {
                    Ok(job) => {
                        println!("Worker {id} got a job; executing");
                        job();
                    }
                    Err(_) => { // Graceful Shutdown
                        println!("Worker {id} dissconnected; shutting down");
                        break;
                    }
                }
            }
        }); // todo: change to std::thread:Builder

        Worker { id, thread }
    }
}

pub enum ThreadPoolError {
    SizeZero,
}

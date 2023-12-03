use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}
impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();
            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing");
                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down");
                    break;
                }
            }
            // let job = receiver.lock().unwrap().recv().unwrap();
            // println!("Worker {id} got a job. executing");
            // job();
        });
        Worker { id, thread: Some(thread) }
    }
}

// Create a ThreadPoll struct
// pub struct ThreadPool {
//     threads: Vec<thread::JoinHandle<()>>,
// }
// struct Job;
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}
type Job = Box<dyn FnOnce() + Send + 'static>;
// implement ThreadPol to create a ThreadPoll instace that take a number as argument

impl ThreadPool {
    /// Create a new ThreadPoll
    ///
    /// The size is the number of threads in the pool
    ///
    /// #Panics
    ///
    /// The new function will panic if the size is zero
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, reciver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(reciver));
        let mut threads = Vec::with_capacity(size);
        for i in 0..size {
            let worker = Worker::new(i, Arc::clone(&receiver));
            threads.push(worker);
        }
        ThreadPool {
            workers: threads,
            sender:Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job);
    }



}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.workers{
            println!("Shuting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take(){
                thread.join().unwrap();
            };

        }
    }
}
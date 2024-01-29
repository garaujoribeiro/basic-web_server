use std::{ sync::{ mpsc::{ self, Receiver }, Arc, Mutex }, thread };

pub struct ThreadPool {
    pub workers: Vec<Worker>,
    pub sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::<Worker>::with_capacity(size);
        for id in 0..size {
            let worker = Worker::new(id, Arc::clone(&receiver));
            workers.push(worker);
        }
        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static {
        let job = Box::new(f);

        match self.sender.send(job) {
            Ok(function) => function,
            Err(e) => panic!("{}", e),
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct Worker {
    pub id: usize,
    pub thread: thread::JoinHandle<()>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();
                println!("Worker {} got a job; executing", id);
                job();
            }
        });

        Worker { id, thread }
    }
}

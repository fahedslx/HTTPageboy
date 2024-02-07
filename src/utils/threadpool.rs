use std::thread::{self, JoinHandle};
use std::sync::{Arc, Mutex, mpsc::{Sender, Receiver, channel}};


type Job = Box<dyn FnOnce() + Send + 'static>;


#[allow(dead_code)]
struct Worker {
	id: usize,
	thread: JoinHandle<()>,
}


impl Worker {
	fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
		let thread = thread::spawn(
			move || loop {
			let job = receiver
				.lock()
				.unwrap()
				.recv()
				.unwrap();
			job();
			}
		);
		return Worker { id, thread };
	}
}


#[allow(dead_code)]
pub struct ThreadPool{
	workers: Vec<Worker>,
	sender: Sender<Job>,
}


impl ThreadPool {
	pub fn new(size: usize) -> ThreadPool {
		assert!(size > 0);

		let (sender, receiver) = channel();
		let receiver = Arc::new(Mutex::new(receiver));
		let mut workers = Vec::with_capacity(size);

		for id in 0..size {
			workers.push(Worker::new(id, Arc::clone(&receiver)));
		}

		return ThreadPool { workers, sender };
	}


	pub fn execute<F>(&self, _f: F)
		where
			F: FnOnce() + Send + 'static
	{
		let job = Box::new(_f);
		self.sender.send(job).unwrap();
	}
}

use std::sync::{ Arc, Mutex };
use std::sync::mpsc::{ Sender, Receiver, channel, SendError };
use std::thread::{ self, JoinHandle };


type Job = Box<dyn FnOnce() + Send + 'static>;


#[allow(dead_code)]
struct Worker {
	id: usize,
	thread: JoinHandle<()>,
}


impl Worker {
	fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
		let thread = thread::spawn(move || {
			loop {
				let job = match receiver.lock() {
					Ok(lock) => match lock.recv() {
						Ok(job) => job,
						Err(_) => break,
					},
					Err(_) => break,
				};
				job();
			}
		});
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

		ThreadPool { workers, sender }
	}


	pub fn execute<F>(&self, _f: F)
		where
			F: FnOnce() + Send + 'static
	{
		let job = Box::new(_f);
		if let Err(err) = self.sender.send(job) {
			match err {
				SendError(_) => println!("Error sending job to thread pool.")
			}
		}
	}
}

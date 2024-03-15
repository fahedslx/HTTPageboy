use std::fmt::{ Display, Formatter, Result };
use std::sync::{ Arc, Mutex };
use std::sync::mpsc::{ Sender, Receiver, channel, SendError };
use std::thread::{ self, JoinHandle };


type Job = Box<dyn FnOnce() + Send + 'static>;
enum Message {
	NewJob(Job),
	Terminate,
}


#[allow(dead_code)]
struct Worker {
	id: usize,
	thread: Option<JoinHandle<()>>,
}


impl Worker {
	fn new(id: usize, receiver: Arc<Mutex<Receiver<Message>>>) -> Worker {
		let thread = thread::spawn(move || {
			loop {
				let message = match receiver.lock() {
					Ok(lock) => match lock.recv() {
						Ok(message) => message,
						Err(_) => break,
					},
					Err(_) => break,
				};

				match message {
					Message::NewJob(job) => {
						println!("Worker {} got a job; executing.", id);
						job();
					}
					Message::Terminate => {
						println!("Worker {} was told to terminate.", id);
						break;
					}
					
				}
			}
		});
		return Worker { id, thread: Some(thread) };
	}
}


#[allow(dead_code)]
pub struct ThreadPool{
	workers: Vec<Worker>,
	sender: Sender<Message>,
}


impl Display for ThreadPool {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "ThreadPool {}", self.workers.len())
	}
}


impl ThreadPool {
	pub fn new(size: usize) -> ThreadPool {
		assert!(size > 0);

		let (sender, receiver) = channel();
		let receiver: Arc<Mutex<Receiver<Message>>> = Arc::new(Mutex::new(receiver));
		let mut workers: Vec<Worker> = Vec::with_capacity(size);

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
		if let Err(err) = self.sender.send(Message::NewJob(job)) {
			match err {
				SendError(_) => println!("Error sending job to thread pool.")
			}
		}
	}
}

impl Drop for ThreadPool {
	fn drop(&mut self) {
		for _ in &self.workers {
			let _ = self.sender.send(Message::Terminate);
		}

		for worker in &mut self.workers {
			println!("Shutting down worker {}", worker.id);

			if let Some(thread) = worker.thread.take() {
				if let Err(e) = thread.join() {
					println!("Error al unir el hilo: {:?}", e);
				}
			}
		}
	}
}

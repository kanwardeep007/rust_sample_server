use std::fs::File;
use std::io::BufRead;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use std::{
    io::{BufReader, Read, Write},
    net::TcpListener,
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread::{sleep, Thread},
};

/// Create a new ThreadPool.
///
/// The size is the number of threads in the pool.
///
/// # Panics
///
/// The `new` function will panic if the size is zero.
struct ThreadPool;
impl ThreadPool {
    fn start(number: u32, receiver: Arc<Mutex<Receiver<Job>>>) {
        for i in 1..=number {
            let w: Worker = Worker {
                id: i,
                receiver: receiver.clone(),
            };
            w.start_working();
        }
    }
    fn execute<F>(sender: Sender<Job>, func: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Job {
            func: Box::new(func),
        };
        sender.send(job);
    }
}

struct Worker {
    pub id: u32,
    pub receiver: Arc<Mutex<Receiver<Job>>>,
}

impl Worker {
    fn start_working(self) {
        thread::spawn(move || {
            while let Ok(inner) = self.receiver.lock().unwrap().recv() {
                (inner.func as Box<dyn FnOnce() -> ()>)();
            }
        });
    }
}

struct Job {
    pub func: Box<dyn FnOnce() + Send + 'static>,
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8181").unwrap();
    let (sender, receiver): (Sender<Job>, Receiver<Job>) = mpsc::channel();
    let arc_mutex_receiver = Arc::new(Mutex::new(receiver));
    ThreadPool::start(4, arc_mutex_receiver);
    for stream in listener.incoming() {
        ThreadPool::execute(sender.clone(), || {
            let mut strm = stream.unwrap();
            let mut buf_reader = BufReader::new(&strm);
            let mut buffer = String::new();
            let request: Vec<_> = buf_reader
                .lines()
                .map(|item| item.unwrap())
                .take_while(|x| !x.is_empty())
                .collect();
            // let write_buf = "HTTP/1.1 200 OK\r\n\r\n";
            // thread::sleep(Duration::from_secs(4));
            let mut hello_html = File::open("hello.html").unwrap();
            let mut write_buffer = String::new();
            let length = hello_html.read_to_string(&mut write_buffer).unwrap();
            strm.write_all(
                format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {length}\r\n\r\n{}",
                    write_buffer
                )
                .as_ref(),
            );
            println!("{:?}", request);
        });
    }
}

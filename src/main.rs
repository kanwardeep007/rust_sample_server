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
    fn start(number: u32, receiver: Arc<Mutex<Receiver<Box<dyn FnOnce() + Send + 'static>>>>) {
        for i in 1..=number {
            let w: Worker = Worker {
                id: i,
                receiver: receiver.clone(),
            };
            w.start_working();
        }
    }
    fn execute<F>(sender: Sender<Box<dyn FnOnce() + Send + 'static>>, func: F)
    where
        F: FnOnce() + Send + 'static,
    {
        sender.send(Box::new(func));
    }
}

struct Worker {
    pub id: u32,

    pub receiver: Arc<Mutex<Receiver<Box<dyn FnOnce() + Send + 'static>>>>,
}

impl Worker {
    fn start_working(self) {
        thread::spawn(move || {
            while let Ok(inner) = self.receiver.lock().unwrap().recv() {
                (inner as Box<dyn FnOnce() -> ()>)();
            }
        });
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8181").unwrap();
    let (tx, rx): (
        Sender<Box<dyn FnOnce() + Send + 'static>>,
        Receiver<Box<dyn FnOnce() + Send + 'static>>,
    ) = mpsc::channel();
    let arc_mutex_receiver = Arc::new(Mutex::new(rx));
    ThreadPool::start(4, arc_mutex_receiver);
    for stream in listener.incoming() {
        ThreadPool::execute(tx.clone(), || {
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

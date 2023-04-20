use crate::common::ServerTrait;
use std::fs::File;
use std::io::BufRead;
use std::thread;
use std::{
    io::{BufReader, Read, Write},
    net::TcpListener,
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
};

/// Create a new ThreadPool.
///
/// The size is the number of threads in the pool.
///
/// # Panics
///
/// The `new` function will panic if the size is zero.
pub struct ThreadPool;
impl ThreadPool {
    fn start(number: u32, receiver: Arc<Mutex<Receiver<Job>>>) -> anyhow::Result<()> {
        for i in 1..=number {
            let w: Worker = Worker {
                _id: i,
                receiver: receiver.clone(),
            };
            w.start_working()?;
        }
        Ok(())
    }
    fn execute<F>(sender: Sender<Job>, func: F) -> anyhow::Result<()>
    where
        F: FnOnce() -> anyhow::Result<()> + Send + Sync + 'static,
    {
        let job = Job {
            func: Box::new(func),
        };
        sender.send(job)?;
        Ok(())
    }
}

struct Worker {
    pub _id: u32,
    pub receiver: Arc<Mutex<Receiver<Job>>>,
}

impl Worker {
    fn start_working(self) -> anyhow::Result<()> {
        thread::spawn(move || {
            while let Ok(inner) = self.receiver.lock().unwrap().recv() {
                (inner.func as Box<dyn FnOnce() -> anyhow::Result<()>>)();
            }
        });
        Ok(())
    }
}

struct Job {
    pub func: Box<dyn FnOnce() -> anyhow::Result<()> + Send + Sync + 'static>,
}

pub struct Approach1Server {
    threads_num: u32,
}

impl Approach1Server {
    pub fn new(threads: u32) -> Self {
        Approach1Server {
            threads_num: threads,
        }
    }
}

impl ServerTrait for Approach1Server {
    fn start_listening(&self, listener: TcpListener) -> anyhow::Result<()> {
        let (sender, receiver): (Sender<Job>, Receiver<Job>) = mpsc::channel();
        let arc_mutex_receiver = Arc::new(Mutex::new(receiver));
        ThreadPool::start(self.threads_num, arc_mutex_receiver)?;
        for stream in listener.incoming() {
            ThreadPool::execute(sender.clone(), || {
                let mut strm = stream.unwrap();
                let buf_reader = BufReader::new(&strm);
                let request: Vec<_> = buf_reader
                    .lines()
                    .map(|item| item.unwrap())
                    .take_while(|x| !x.is_empty())
                    .collect();

                // thread::sleep(Duration::from_secs(4));
                let request_string = &request.first().unwrap()[0..=3];
                let mut content = "";
                if request_string == "GET " {
                    content = "Got a simple get request";
                } else if request_string == "POST" {
                    content = "Oh Noo post req";
                };

                let length = content.len();
                strm.write_all(
                    format!(
                        "HTTP/1.1 200 OK\r\nContent-Length: {length}\r\n\r\n{}",
                        content
                    )
                    .as_ref(),
                )?;
                println!("{:?}", request);
                Ok(())
            })?;
        }
        Ok(())
    }
}

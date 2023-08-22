use std::net::TcpListener;
use super::request::Request;
use std::thread;
use std::sync::{mpsc, Arc, Mutex};



#[derive(Debug)]
pub struct Server{
    ip: &'static str,
    port: &'static str,
}

impl Server{
    pub fn new(s: &'static str) -> Self{
        let (ip, port) = s.split_once(":").unwrap();
        return Self{ip, port};
    }

    pub fn run(self){
        let listner = match TcpListener::bind(self.ip.to_string() + ":"+self.port){
            Ok(listner)=> listner,
            Err(_) => panic!("can`t to listning by this address"),
        };
        let pool = ThreadPool::new(4);

        for stream in listner.incoming(){
            let stream = stream.unwrap();
            pool.execute(|| {
                Request::request_handler(stream);
            })
        }
    }
}

struct ThreadPool{
    threads: Vec<Worker>,
    sender: mpsc::Sender<Job>
}


impl ThreadPool{
    fn new(size: usize)->Self{
        assert!(size>0);
        let mut threads: Vec<Worker> = Vec::with_capacity(size);

        let (sender, receiver) = mpsc::channel();
        
        let receiver = Arc::new(Mutex::new(receiver));
        for a in 0..size{
            threads.push( Worker::new(a, Arc::clone(&receiver)));
        }
        Self { threads, sender }

    }

    fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F{
    fn call_box(self: Box<F>){
        (*self)()
    }
}

type Job = Box<dyn FnBox + Send + 'static>;

struct Worker{
    id: usize,
    thread: thread::JoinHandle<()>
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>)-> Self{
        let thread = thread::spawn(move ||{
            loop{
                let job = receiver.lock().unwrap().recv().unwrap();
                println!("Worker {} get a task; task in comming ", id);
                job.call_box()
            }
        });
        Self{id, thread}
    }
}

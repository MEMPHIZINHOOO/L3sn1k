/// AUXILIARY FUNCTIONS FOR DEVELOPMENT
//aux imports
// threading imports
// CLI imports
// proxy imports 

use std::{sync::{Arc, Mutex, mpsc}, thread, io::{BufReader, prelude::*}, net::{TcpStream, TcpListener}};


/// class responsible for dealing with threadPooling and workers queues
/// it is abstracted from the developer, should be easy to use, might modify it a bit later on
/// it's pretty weird rust stuff, did you know mutex safe queues do not exist normally?
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>, 
}
 

impl ThreadPool {
    /// creates a new instance of threadPool
    /// it's a contained [`thread array`] that contains a limited amount of threads
    /// it works functionally like a queue, take a bit of information
    /// execute over it
    ///
    pub fn execute<F>(&self, f:F)
    where 
        F: FnOnce() + Send + 'static,
        {
            let job = Box::new(f);

            self.sender.as_ref().unwrap().send(job).unwrap();
        }
    /// builds a new threadPool, takes a [`usize`] size and returns a threadpool with those corresponding
    /// number of threads, the lower amount of threads means a more runnable app, but also a weaker proxy
    /// in theory it shouldn't require much unless you want to increase througoutput
    ///
    /// # Examples
    /// ```
    /// fn main() {
    ///   pool = ThreadPool::new(4);
    ///   let array_of_print Vec<_>;
    ///   for id in 0..100 {
    ///   array_of_print.push( || println!("function"));
    ///   }
    ///   pool.execute(|| {
    ///      let print = array_of_print[0..]
    ///      print();
    ///   })
    /// }
    /// ```
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver ) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool { workers, sender: Some(sender),}
    }

}
/// representation of a function F
type Job = Box<dyn FnOnce() + Send + 'static>;

/// the Worker is responsible for looping the built threads and keepign them stuck until a Job is received
/// a Job is nothing more than the representation of any type F function
/// it is NOT directly callable by the developer, but you can use it, I guess
/// 
/// [`receiver`] is a Arc queue that holds a receiver side to the function being called
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv();

                match message {
                    Ok(job) => job(),
                    Err(_) =>  break,
                }
            }
        });

        Worker {id, thread}
    }
}
/// contains the implementation of clean Dropping for smooth shutdown
impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in  self.workers.drain(..) {
            println!(" Shutting down worker {}", worker.id);

            worker.thread.join().unwrap();
        }
    }
}

/// Cli commands abstractions

/// Connection type, it corresponds to all the information passing through a specific thread [`Worker`]
/// it is constructed by each and every thread for each connection and can be applied modifications to in the
/// repeater tab, every single connection is kept in ram till explicitly declared to be deleted, in such scenario
/// it gets deleted out of the queue and killed
/// contains [`request`] which has pretty much all important information to be sent to the frontend, the rest is all internal 
pub struct Connection<'a> {
    pub peer_address: std::net::SocketAddr,
    pub request: std::io::Lines<BufReader<&'a TcpStream>>,
    pub domain: &'a str,
}
/// still to be built, will communicate with a thread responsible for communicating with the application
pub struct ConnectionQueue<'a> {
    pub receiver: Option<mpsc::Sender<Connection<'a>>>, 
}



pub fn proxy_run<'a>(arg: usize) {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = ThreadPool::new(arg);
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute( || {
            handle_connection(stream);
        });
    }
}

fn handle_connection<'a>(stream: TcpStream) {
    //  still to be done establishTLS()
    let peer_address = stream.peer_addr().unwrap();
    let buf_reader = BufReader::new(&stream);
    let mut request = buf_reader.lines();
    
    let host_line = request.nth(1);
    let domain = host_line.unwrap().unwrap().get(6..).unwrap().to_string();


    
    let _thread_connection = Connection { peer_address: peer_address, request, domain: &domain};


    
}

// should do something
fn interact_with_app() {
    return    
}

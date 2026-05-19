//aux imports
use aux::ThreadPool;
// threading imports
use std::thread;
// CLI imports
use clap::Parser;

// proxy imports 
use std::{
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

#[derive( Clone, Parser)]
struct Cli {
    pattern: String,
    address: String,
}
/// runs the proxy, it is built with a lifetime parameter to guarantee the proxy is not
/// always running, it is by default [` 'static`]. takes [`args: Cli` struct] as an argument
///
/// in theory it returns nothing, it is simply executed by a thread
fn proxy_run<'a>(args: Cli) {
    let listener = TcpListener::bind(args.address).unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        
        handle_connection(stream);
    }
}
fn main() {
    ctrlc::set_handler(move || {
        println!("shutting down proxy");
        std::process::exit(0);
    });
    let args = Cli::parse();
    thread::spawn(|| {
        proxy_run(args);
    });
    
}

/// handles connection, it is built with a lifetime parameter to guarantee exeuction within a certain
/// time constraint, that time constraint is decided by the encapsulation function [`proxy_run`].
///
/// however it can be statically defined
fn handle_connection<'a>(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

}

//aux imports
use aux::{ThreadPool, Connection, Cli};
// threading imports
// CLI imports
use clap::Parser;
// proxy imports 
use std::{
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

/// runs the proxy, it is built with a lifetime parameter to guarantee the proxy is not
/// always running, it is by default [` 'static`]. takes [`args: Cli` struct] as an argument
///
/// in theory it returns nothing, it is simply executed by a thread
fn proxy_run<'a>(args: Cli) {
    let listener = TcpListener::bind(args.address).unwrap();
    let pool = ThreadPool::new(args.threads.parse::<usize>().expect("failed to parsed Cli thread argument"));
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute( || {
            handle_connection(stream);
        });
    }
}
fn main() {
    let handle_result = ctrlc::set_handler(move || {
        println!("shutting down proxy");
        std::process::exit(0);
    });

    
    match handle_result {
        Ok(_) => (),
        Err(err) => panic!("Error: {}", err),
    }
    
    let args = Cli::parse();
    thread::spawn(|| {
        proxy_run(args);
    });

    thread::spawn(|| {
        interact_with_app();
    });
    loop {
        std::thread::sleep(Duration::from_secs(1));
    }
}

/// handles connection, it is built with a lifetime parameter to guarantee exeuction within a certain
/// time constraint, that time constraint is decided by the encapsulation function [`proxy_run`].
///
/// however it can be statically defined
fn handle_connection<'a>(stream: TcpStream) {
    //  still to be done establishTLS()
    let peer_address = stream.peer_addr().unwrap();
    let buf_reader = BufReader::new(&stream);
    let mut request = buf_reader.lines();
    
    let host_line = request.nth(1);
    let domain = host_line.unwrap().unwrap().get(6..).unwrap().to_string();


    
    let _thread_connection = Connection { peer_address: peer_address, request, domain: &domain};
    
}


/// opens a file and interacts with it, keeps track of connection changes, interactions and differences
/// 
fn interact_with_app() {
    return    
}

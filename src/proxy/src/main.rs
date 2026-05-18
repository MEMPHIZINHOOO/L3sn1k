use std::ffi::CString;
// threading imports
use std::thread;
use std::time::Duration;

// CLI imports
use clap::Parser;


// proxy imports 
use std::net::TcpListener

#[derive(Parser)]
struct Cli {
    pattern: String,
    port: String,
}

fn proxy_run(pattern: String, port: String) {
    let listener = TcpListener::bind(port)

    for stream in listener.incoming() {
        let stream = stream.unwrap_or();
    }
}
fn main() {
    let args = Cli::parse();
    thread::spawn(|| {
        proxy_run(args.pattern ,args.port);
    });
    println!("proxy is running on port {}", &args.port);

    
}

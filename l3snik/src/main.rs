
mod gui;
mod tests;

//imports
use std::env;
use std::net::{SocketAddr,IpAddr, Ipv4Addr};
use std::thread::sleep;
use std::time::Duration;

use tools::proxy::{instantiate_proxy};
use tools::logging::log_and_fix;

use tokio::main;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;

use anyhow::Error;

#[main]
async fn main() {
        
    let mut arguments: Vec<String> = env::args().collect();
    let port_input = arguments.pop();
    let address: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port_input.expect("not a valid port").parse().expect("not a valid port"));

    // channel for communication from gui to proxy    
    let (tx_sender_proxy_info, tx_receiver_proxy_info) = mpsc::channel(1);

    // channel for communication from proxy, gui, and any other async info
    // into logging task
    let (tx_sender_error_info, tx_receiver_error_info) : (
    Sender<Error>,
    Receiver<Error>
) = mpsc::channel(5);

    //multiple, and I mean, plenty of tasks, will be sending information for logging purposes
    // the log will, serve as the main head of the project and deal with errors AND
    // processing our results from our tools, a big pain in the ass is coming ahead
    
    
    tokio::spawn(instantiate_proxy(address, tx_sender_proxy_info, tx_sender_error_info.clone()));
    tokio::spawn( log_and_fix(tx_receiver_error_info));

    unsafe {
        std::env::set_var("WGPU_POWER_PREF", "high");
    }
    
    match gui::L3snikGui::start(tx_receiver_proxy_info, tx_sender_error_info.clone()) {
        Ok(_) => {println!("shutting down now");},
        Err(_) => {println!("waiting for restart"); sleep(Duration::from_secs(1));}
    }
} 

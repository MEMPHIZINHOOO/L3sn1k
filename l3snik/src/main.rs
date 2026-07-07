
mod gui;
mod tests;

use anyhow::Error;
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

#[main]
async fn main() {
        
    let mut arguments: Vec<String> = env::args().collect();
    let debug = arguments.pop();
    let port_input = arguments.pop();

    // verifying for argument
    // keep in mind the debug command currently is being verified for the proxy
    // requests alone, it DOES NOT work for debugging gui related commands
    if let Some(ref debug_string) = debug {
        if *debug_string != "debug".to_string() {
            println!("the second argument is invalid.");
            std::process::exit(0);
        }
    }

    let address: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port_input.expect("not a valid port").parse().expect("not a valid port"));

    // channel for communication from gui to proxy    
    let (tx_sender_gui_proxy_info, tx_receiver_gui_proxy_info) = mpsc::channel(1);

    //wacky verification for deploying the channel respective to debugging
    let (tx_sender_log_proxy_info_option, tx_receiver_log_proxy_info_option) = match debug {
        Some(_) =>  { let (tx, rx) = mpsc::channel(1); (Some(tx), Some(rx)) }
        None => (None, None),
    };
    // channel for communication from proxy, gui, and any other async info
    // into logging task
    let (tx_sender_error_info, tx_receiver_error_info) : (
    Sender<Error>,
    Receiver<Error>
) = mpsc::channel(5);

    //multiple, and I mean, plenty of tasks, will be sending information for logging purposes
    // the log will, serve as the main head of the project and deal with errors AND
    // processing our results from our tools, a big pain in the ass is coming ahead
    
    if let Some(tx_sender_log_proxy_info) = tx_sender_log_proxy_info_option {
        tokio::spawn(instantiate_proxy(address, tx_sender_log_proxy_info, tx_sender_error_info.clone()));
        tokio::spawn(log_and_fix(tx_receiver_error_info,tx_receiver_log_proxy_info_option));
    }
    else {
        tokio::spawn(instantiate_proxy(address, tx_sender_gui_proxy_info, tx_sender_error_info.clone()));
        tokio::spawn( log_and_fix(tx_receiver_error_info, None));
        
    }

    unsafe {
        std::env::set_var("WGPU_POWER_PREF", "high");
    }
    
    match gui::L3snikGui::start(tx_receiver_gui_proxy_info, tx_sender_error_info.clone()) {
        Ok(_) => {println!("shutting down now");},
        Err(_) => {println!("waiting for restart"); sleep(Duration::from_secs(1));}
    }
} 

// use std::sync::mpsc;
use std::env;
use std::net::{SocketAddr,IpAddr, Ipv4Addr};
use std::error::Error;
use tools::proxy::instantiate_proxy;

mod gui;
mod tests;

use tokio::main;
use tokio::sync::mpsc;

#[main]
async fn main() -> Result<(), Box<dyn Error>> {
    
    let mut arguments: Vec<String> = env::args().collect();
    let port_input = arguments.pop();
    let address: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port_input.expect("not a valid port").parse()?);

    // these channels will be useful, soon enough
    let (tx_sender, _tx_receiver) = mpsc::channel(1); 
    tokio::spawn(instantiate_proxy(address, tx_sender));

    unsafe {
        std::env::set_var("WGPU_POWER_PREF", "high");
    }

    gui::L3snikGui::start()
} 

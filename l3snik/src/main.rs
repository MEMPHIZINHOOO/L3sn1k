// use std::sync::mpsc;
use std::env;
use std::thread;
use std::net::SocketAddr;

use tools::proxy::instantiate_proxy;

mod gui;
mod tests;

use tokio::sync::mpsc;
use proxelar::ProxyEvent;

fn main() -> iced::Result {
    
    let mut arguments: Vec<String> = env::args().collect();
    let port_input = arguments.pop();
    let address: SocketAddr = SocketAddr::new(SocketAddrV4::new(127, 0, 0, 1), port_input.parse().expect("not a valid port"));

    let (tx_sender, tx_receiver) = mpsc::channel(1); 
    thread::spawn(||  {instantiate_proxy(address, tx_sender)});

    unsafe {
        std::env::set_var("WGPU_POWER_PREF", "high");
    }

    gui::L3snikGui::start()
} 

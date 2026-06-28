// use std::sync::mpsc;
use std::env;
use std::thread;
use tools::proxy::proxy_run;
mod gui;
mod tests;
fn main() -> iced::Result {
    let mut arguments: Vec<String> = env::args().collect();
    let thread_number = arguments.pop();

    thread::spawn(||  {proxy_run(thread_number.unwrap_or("4".to_string()).parse::<usize>().expect("no thread number specified"))});

    unsafe {
        std::env::set_var("WGPU_POWER_PREF", "high");
    }

    gui::L3snikGui::start()
} 

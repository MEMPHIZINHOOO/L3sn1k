use core::fmt;
use std::{ error::Error, 
    fs::{self, DirEntry,},
    net::SocketAddr,
    path::{Path, PathBuf},
    sync::{Arc, Mutex}
};

use proxelar::{Proxy, ProxyConfig, ProxyEvent, UpstreamTlsConfig};
use tokio::sync::mpsc::Sender;
use certgenutil::{ load_cert_from_pem_file};


#[derive(Debug)]
struct ProxyErrors {
    error: String,
}

const CA_DEFAULT_DIR: &'static str = "./.keys";


impl fmt::Display for ProxyErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // let error = self.error.
        let error = &self.error;
        write!(f, "{error}")
    }
}

impl Error for ProxyErrors {}

//resolve both functions to return both errors, #1
pub async fn instantiate_proxy(arg: SocketAddr, tx_proxy: Sender<ProxyEvent>, tx_log: Arc<Mutex<Sender<Box<dyn Error + Send>>>>) -> Result<String, ()> 
{
    let mut resolved_path = PathBuf::new();
    // we are not resolving for env error, we probably need a separate function or a more hardened resolve, the path should exist by the time we try to actually open it
    PathBuf::push(&mut resolved_path, std::env::var("CA_DIR").unwrap_or(CA_DEFAULT_DIR.to_string()));
    
    let proxy_config = ProxyConfig {
        addr: arg,
        mode: proxelar::ProxyMode::Forward,
        event_tx: tx_proxy,
        ca_dir: resolved_path,
        upstream_tls: UpstreamTlsConfig::Default,
        //@TODO: get the interception communicating
        intercept: None,
        body_capture_limit: None,
        //@TODO: might also come in handy
        replay_rx: None,
    };
    let proxy = Proxy::new(proxy_config);

    if let Err(_) =proxy.start( async { }).await {
        let held_tx_lock = tx_log.lock().unwrap().take().expect("logger has been called more than once");
        //@todo: fix improper error handling, proxy error should be able to grab the Err from inside proxelar and cast it upwards in our own notation
        tx_log.try_send(Box::new(ProxyErrors { error: "proxy failed to start or failed during execution".to_string()})).expect("the logging is not working properly");
        ()
    }
    
    Ok("turning proxy off".to_string())       
}

/// auxiliary function to verify that everything is working properly, currently, this does NOT have support into the ongoing app, it is here for debugging and testing purposes
// #[warning(dead_code)]
pub fn check_init_ca_dir() -> Result<(), Box<dyn Error>> {
    // we read the dir
    let checks: Vec<String> = vec!["key.pem".to_string(), "key.ca".to_string()];

    // obviously we do not know whether the CA_DIR exists in the environment, but if not, we default to unwrapping the default value
    // this will deal a panic in case there's no resolution, because it is not supposed to either way

    if  Path::new(std::env::var("CA_DIR")?.as_str()).exists() {
        if let Ok(directory) = fs::read_dir(std::env::var("CA_DIR").unwrap_or(CA_DEFAULT_DIR.to_string())){
            for entry in directory {

                let entry: DirEntry = entry?;
                let entry_path: PathBuf = DirEntry::path(&entry);

                //@todo: we need to handle this error better

                let name: String = match DirEntry::file_name(&entry).into_string() {
                    Ok(entry_name) => entry_name,
                    Err(_) => return Err(Box::new(ProxyErrors { error: "failed to open directory".to_string()})),
                };

                if checks.contains(&name)
                    {
                        //@todo: add verification of proper CA, and proper error handling«
                        match load_cert_from_pem_file(entry_path) {
                            Ok(file_cert) => file_cert,
                            Err(_) => return Err(Box::new(ProxyErrors {error: "failed to load certificate from file, file might be corrupted".to_string()})),
                        };
                    
                    }                
            }
        
            if checks.len() != 0 {
                return Err(Box::new(ProxyErrors { error: "the directory had files deleted from them that are essential, please remove the directory".to_string()}));
            }
            return Ok(());  
        }
        // looks good both files are verified, however, we don't confirm whether they actually verify
        // to what we want, proxelar takes care of that in theory
        panic!("not supposed to happen, wasn't able to read existing dir");
    }
    panic!("path does not exist")
             
}

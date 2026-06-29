use std::io;
use std::path::Path;
use std::{ net::SocketAddr, path::PathBuf, fs::{self, DirEntry}};
use proxelar::{Error, Proxy, ProxyConfig, ProxyEvent, UpstreamTlsConfig};
use tokio::sync::mpsc::Sender;
use certgenutil::generate_self_signed_cert;

//resolve both functions to return both errors, #1
pub async fn instantiate_proxy(arg: SocketAddr, tx: Sender<ProxyEvent>) -> Result<String, Error> 
{
    let mut resolved_path = PathBuf::new();
    // we are not resolving for env error, we probably need a separate function or a more hardened resolve, the path should exist by the time we try to actually open it
    PathBuf::push(&mut resolved_path, std::env::var("CA_DIR").unwrap_or("../../../.keys".to_string()));
    
    let proxy_config = ProxyConfig {
        addr: arg,
        mode: proxelar::ProxyMode::Forward,
        event_tx: tx,
        ca_dir: resolved_path,
        upstream_tls: UpstreamTlsConfig::Default,
        //@TODO: get the interception communicating
        intercept: None,
        body_capture_limit: None,
        //@TODO: might also come in handy
        replay_rx: None,
    };
    let proxy = Proxy::new(proxy_config);

    proxy.start( async { }).await?;
    Ok("turning proxy off".to_string())       
}

pub fn check_init_CA_dir() -> Result<(), Error> {
    // we read the dir
    if let Ok(directory) = fs::read_dir(std::env::var("CA_DIR").unwrap_or("../../../.keys".to_string())) {
        return Ok(());  
    }
    else {
        return Ok(());
    }
             
}

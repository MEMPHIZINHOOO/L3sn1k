use std::{net::SocketAddr, path::PathBuf};
use std::path;
use proxelar::{Proxy, ProxyConfig, UpstreamTlsConfig, ProxyEvent};
use tokio::sync::mpsc::Sender;

pub async fn instantiate_proxy(arg: SocketAddr, tx: Sender<ProxyEvent>) -> Result<(), T: std::error::Error> {
    let mut resolved_path = PathBuf::new();
    PathBuf::push(&mut resolved_path, std::env::var("CA_DIR")?);
    
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

    proxy.start(True).await?;
    Ok("turning proxy off");        
}




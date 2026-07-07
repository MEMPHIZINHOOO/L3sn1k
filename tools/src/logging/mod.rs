/// auxiliary tools responsible for error handling and trying to fix potential issues
/// this exists to ensure there are no problems in runtime and if they do happen we try and restart them to fix the issue whilst the app is running
/// this way we prevent cascading failures
///
///
///

use anyhow::Error;
use proxelar::ProxyEvent;
use tokio::sync::mpsc::Receiver;


/// log and fix is a big function that tries to resolve all errors
/// most of the times by throwing, for example, the proxy, back into
/// running, this is relevant for many reasons.
pub async fn log_and_fix(mut error_receiver: Receiver<Error>, proxy_receiver_option: Option<Receiver<ProxyEvent>>) {
    if let Some(mut proxy_receiver) = proxy_receiver_option {
        loop {
            tokio::select! {
                val = error_receiver.recv() => { eprintln!("{val:?}");}
                // print debug feature for seeing whether it grabs requests
                _val = proxy_receiver.recv() => { println!("got a request");}
            }
        }    
    }

    else { 
        loop {
        //@TODO  still to be implemented
            match error_receiver.recv().await {
                Some(error) => eprintln!("{error:?}"),
                    // malformed http request
                _ => (),
                }
        }
    }
}
        

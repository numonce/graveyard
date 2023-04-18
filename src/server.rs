use std::net::TcpListener;
use std::sync::mpsc::{Receiver, Sender};
pub fn start(tx: Sender<String>, rx: Receiver<String>) -> Result<(), Box<dyn std::error::Error>> {
    let server = TcpListener::bind("127.0.0.1:1337")?;
    loop {
        match rx.try_recv() {
            Ok(_) => break,
            Err(_) => continue,
        };
        for stream in server.incoming() {
            tx.send(stream?.peer_addr()?.to_string())?;
        }
    }
    Ok(())
}

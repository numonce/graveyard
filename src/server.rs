use std::net::TcpListener;
use std::sync::mpsc::{Receiver, Sender};
pub fn start(tx: Sender<String>, rx: Receiver<String>) -> Result<(), Box<dyn std::error::Error>> {
    let server = TcpListener::bind("127.0.0.1:1337")?;
    server.set_nonblocking(true)?;
    loop {
        match server.accept() {
            Ok((_socket, addr)) => tx.send(addr.to_string())?,
            Err(_) =>{
        match rx.try_recv() {
            Ok(_) => break,
            Err(_) => continue,
        };},
        };
    }
    Ok(())
}

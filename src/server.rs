use std::io::{ErrorKind, Read};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{Receiver, Sender};
use std::thread::spawn;
pub fn start(tx:  Sender<String>, rx: Receiver<String>) -> Result<(), Box<dyn std::error::Error>> {
    let server = TcpListener::bind("127.0.0.1:1337")?;
    server.set_nonblocking(true)?;
    for stream in server.incoming() {
        let transmit = tx.clone();
        match stream {
            Ok(mut socket) => spawn(move || { handle_tcp_connection(&mut socket, transmit).unwrap();}),
            Err(_) => {
                match rx.try_recv() {
                    Ok(_) => break,
                    Err(_) => continue,
                };
            }
        };
    }
    Ok(())
}

fn handle_tcp_connection(
    stream: &mut TcpStream,
    tx: Sender<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    //Send notification to append connection to list.
    let add_connection = String::from("add ") + &stream.peer_addr()?.to_string();
    tx.send(add_connection)?;
    // keeping the connection alive. This is for testing. 
    let mut buffer = [0; 1024];
    match stream.read(&mut buffer) {
        Ok(bytes_read) => {
            println!("Read {:?}", &buffer[..bytes_read]);
        }
        Err(e) if e.kind() == ErrorKind::ConnectionAborted => {
            println!("Other side disconnected");
        }
        Err(e) => {
            println!("Some other error occurred: {e}");
        }
    }
    //Remove the connection from the list.
    let del_connection = String::from("del ") + &stream.peer_addr()?.to_string();
    tx.send(del_connection)?;
    Ok(())
}

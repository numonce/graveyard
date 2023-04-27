use std::net::{TcpListener, TcpStream};

use s2n_quic::Connection;
use flume::{Sender,Receiver};
use tokio::select;
pub async fn start(
    tx: Sender<String>,
    rx: Receiver<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let server = TcpListener::bind("127.0.0.1:1337")?;
    server.set_nonblocking(true)?;
    for stream in server.incoming() {
        let transmit = tx.clone();
        match stream {
            Ok(mut socket) => {
                tokio::spawn(async move {
                    handle_tcp_connection(&mut socket, transmit).unwrap();
                })
                .await
            }
            Err(_) => {
                match rx.try_recv() {
                    Ok(_) => break,
                    Err(_) => continue,
                };
            }
        }?;
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
    //Remove the connection from the list.
    let del_connection = String::from("del ") + &stream.peer_addr()?.to_string();
    tx.send(del_connection)?;
    Ok(())
}

pub async fn start_quic(tx: Sender<String>, rx: Receiver<String>) -> Result<(),Box<dyn std::error::Error>>{
 let mut quic_server = s2n_quic::Server::builder().with_io("127.0.0.1:1337")?.start()?;
 
 loop {
     let transmit = tx.clone();
 select! {
stream = quic_server.accept() => tokio::spawn(async move {handle_quic_connection(stream.unwrap(), transmit).await.unwrap()}),
_ = rx.recv_async() => break,
 };
 }
Ok(())
}

async fn handle_quic_connection(stream: Connection, tx: Sender<String>) -> Result<(), Box<dyn std::error::Error>>{
println!("we got it!");
let add_connection = String::from("add ") + &stream.remote_addr()?.to_string();
tx.send(add_connection)?;
let del_connection = String::from("del ") + &stream.remote_addr()?.to_string();
tx.send(del_connection)?;
Ok(())
}

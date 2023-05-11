mod server;
mod ui;
use flume::unbounded;
#[tokio::main]

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (uitx, uirx) = unbounded();
    let (tcptx, tcprx) = unbounded();
    let quictx = tcptx.clone();
    let quicrx = uirx.clone();
    let ui_handle = tokio::spawn( async move {
        ui::start_ui(uitx, tcprx).unwrap();
    });
    let tcp_server_handle = tokio::spawn(async move {
        server::start(tcptx, uirx).await.unwrap();
    });
    let quic_server_handle = tokio::spawn(async move {
        server::start_quic(quictx, quicrx).await.unwrap();
    });
    tcp_server_handle.await?;
    quic_server_handle.await?;
    ui_handle.await?;
    Ok(())
}



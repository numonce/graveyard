mod server;
mod ui;
use std::sync::mpsc;
use std::thread::spawn;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();
    let ui_handle = spawn(move || {
        ui::start_ui(tx1, rx2).unwrap();
    });
    let server_handle = spawn(move || {
        server::start(tx2, rx1).unwrap();
    });
    ui_handle.join().expect("Thread panicked");
    server_handle.join().expect("Thread panicked");
    Ok(())
}

mod ui;
use std::thread::spawn;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let handler = spawn( || {
        ui::start_ui().unwrap();
    });
    handler.join().expect("Thread panicked");
    Ok(())
}

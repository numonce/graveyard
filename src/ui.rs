use crossterm::{
    event::{
        read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders},
    Terminal,
};
use std::io;

pub fn start_ui() -> Result<(), io::Error> {
    //setup the keystrokes.
    let ctrl_c = KeyCode::Char('c');
    let cquit = KeyEvent::new(ctrl_c, KeyModifiers::CONTROL);
    let q = KeyCode::Char('q');
    let quit = KeyEvent::new(q, KeyModifiers::NONE);

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Draw the terminal and handle user input.
    loop {
        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default().title("Block").borders(Borders::ALL);
            f.render_widget(block, size);
        })?;
        if crossterm::event::poll(std::time::Duration::from_millis(100))? {
            let keystroke = read()?;
            if keystroke == Event::Key(quit) || keystroke == Event::Key(cquit) {
                break;
            }
        }
    }
    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

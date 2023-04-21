use crossterm::{
    event::{
        read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::style::Color;
use ratatui::{
    backend::CrosstermBackend,
    style::Style,
    widgets::{Block, Borders, List, ListItem, ListState},
    Terminal,
};
use std::io::{self, Stdout};
use std::sync::mpsc::{Receiver, Sender};

pub fn start_ui(
    tx: Sender<String>,
    rx: Receiver<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    //setup the keystrokes.
    let cquit = Event::Key(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL));
    let quit = Event::Key(KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE));
    let up = Event::Key(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));
    let down = Event::Key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnableMouseCapture, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut events = Events::new(Vec::new());
    // Draw the terminal and handle user input.
    loop {
        let items: Vec<ListItem> = events
            .items
            .iter()
            .map(|i| ListItem::new(i.as_ref()))
            .collect();
        //parse the items in list.
        //Create the stylized list.
        let item_list = List::new(items)
            .block(Block::default().title("Beacons").borders(Borders::ALL))
            .style(Style::default().fg(Color::Cyan))
            .highlight_style(Style::default().bg(Color::Magenta))
            .highlight_symbol(">>");
        //Render the box and list.
        terminal.draw(|f| {
            let size = f.size();
            f.render_stateful_widget(item_list, size, &mut events.state);
        })?;
        //Handle user input.
        if crossterm::event::poll(std::time::Duration::from_millis(100))? {
            let keystroke = read()?;
            if keystroke == cquit || keystroke == quit {
                tx.send(String::from("Kill yourself."))?;
                gracefully_exit(terminal)?;
                break;
            } else if keystroke == up {
                events.previous();
            } else if keystroke == down {
                events.next();
            }
        }
        match rx.try_recv() {
            Ok(msg) => {
                let slice = msg.split(" ").collect::<Vec<_>>();
                match slice[0] {
                    "add" => events.items.push(slice[1].to_string()),
                    _ => events
                        .items
                        .retain(|x| x.to_owned() != slice[1].to_string()),
                }
            }
            Err(_) => continue,
        };
    }
    Ok(())
}

pub fn gracefully_exit(
    //
    mut terminal: Terminal<CrosstermBackend<Stdout>>,
) -> Result<(), Box<dyn std::error::Error>> {
    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        DisableMouseCapture,
        LeaveAlternateScreen,
    )?;
    terminal.show_cursor()?;

    Ok(())
}
struct Events {
    // `items` is the state managed by your application.
    items: Vec<String>,
    // `state` is the state that can be modified by the UI. It stores the index of the selected
    // item as well as the offset computed during the previous draw call (used to implement
    // natural scrolling).
    state: ListState,
}

impl Events {
    fn new(items: Vec<String>) -> Events {
        Events {
            items,
            state: ListState::default(),
        }
    }

    // Select the next item. This will not be reflected until the widget is drawn in the
    // `Terminal::draw` callback using `Frame::render_stateful_widget`.
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    // Select the previous item. This will not be reflected until the widget is drawn in the
    // `Terminal::draw` callback using `Frame::render_stateful_widget`.
    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

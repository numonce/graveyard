use serde::{Serialize,Deserialize};
use crossterm::{
    event::{
        read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use flume::{Receiver, Sender};
use ratatui::{
    backend::CrosstermBackend,
    style::Style,
    widgets::{Block, Borders, List, ListItem, ListState, BarChart},
    Terminal,
};
use ratatui::{
    layout::{Constraint::Percentage, Direction, Layout},
    style::Color,
    text::{Span, Spans},
    widgets::{Paragraph, Wrap},
};
pub fn start_ui(
    tx: Sender<String>,
    rx: Receiver<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    //setup the keystrokes.
    let cquit = Event::Key(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL));
    let quit = Event::Key(KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE));
    let up = Event::Key(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));
    let down = Event::Key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    let enter = Event::Key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnableMouseCapture, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut beacons = Beacons::new(Vec::new());
    //Partition out the screen.
    // Draw the terminal and handle user input.
    loop {
        let hchunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Percentage(50), Percentage(50)])
            .split(terminal.size()?);
        let vchunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Percentage(50), Percentage(50)])
            .split(hchunks[1]);
        // Create a list from "beacons"
        let items: Vec<ListItem> = beacons
            .items
            .iter()
            .map(|i| ListItem::new(i.ip.as_ref()))
            .collect();
        //Create the stylized list.
        let item_list = List::new(items)
            .block(Block::default().title("Beacons").borders(Borders::ALL))
            .style(Style::default().fg(Color::Green))
            .highlight_style(Style::default().bg(Color::DarkGray))
            .highlight_symbol(">>");
        //Create instructions
        let instructions = Spans::from(vec![Span::styled(
            "This is a stand in for the general instructions on how to make use of this C2.",
            Style::default().fg(Color::Green),
        )]);
        let instructions_box = Paragraph::new(instructions)
            .block(
                Block::default()
                    .title("help")
                    .title_alignment(ratatui::layout::Alignment::Center)
                    .borders(Borders::ALL),
            )
            .alignment(ratatui::layout::Alignment::Center)
            .wrap(Wrap { trim: true });
        //Create graph. Currently a place holder.
        let barchart = BarChart::default()
    .block(Block::default().title("jitter")
           .title_alignment(ratatui::layout::Alignment::Center).borders(Borders::ALL))
    .bar_width(3)
    .bar_gap(1)
    .bar_style(Style::default().fg(Color::Yellow).bg(Color::Red))
    .value_style(Style::default().fg(Color::Red).add_modifier(ratatui::style::Modifier::BOLD))
    .label_style(Style::default().fg(Color::White))
    .data(&[("B0", 5), ("B1", 2), ("B2", 4), ("B3", 3)])
    .max(4);

        //Render the box and list.
        terminal.draw(|f| {
            f.render_stateful_widget(item_list, hchunks[0], &mut beacons.state);
            f.render_widget(instructions_box, vchunks[0]);
            f.render_widget(barchart, vchunks[1]);
        })?;
        //Handle user input.
        if crossterm::event::poll(std::time::Duration::from_millis(100))? {
            let keystroke = read()?;
            if keystroke == cquit || keystroke == quit {
                tx.send(String::from("Kill yourself."))?;
                std::thread::sleep(std::time::Duration::from_millis(100));
                gracefully_exit(terminal)?;
                break;
            } else if keystroke == up {
                beacons.previous();
            } else if keystroke == down {
                beacons.next();
           }else if keystroke == enter {
               context_menu(&mut terminal)?;
           }
        }
        match rx.try_recv() {
            Ok(msg) => {
                let slice = msg.split(" ").collect::<Vec<_>>();
                match slice[0] {
                    "add" => {
                        let zombie: Zombie = serde_json::from_str(slice[1])?;
                        beacons.items.push(zombie);
                    }
                    _ =>{
                        let zombie: Zombie = serde_json::from_str(slice[1])?;
                        let z_index = beacons.items.iter().position(|x| *x == zombie).unwrap();
                        beacons
                        .items
                        .remove(z_index);
                }
                }
            }
            Err(_) => continue,
        };
    }
    Ok(())

    }
pub fn gracefully_exit(
    //
    mut terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
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
struct Beacons {
    // `items` is the state managed by your application.
    items: Vec<Zombie>,
    // `state` is the state that can be modified by the UI. It stores the index of the selected
    // item as well as the offset computed during the previous draw call (used to implement
    // natural scrolling).
    state: ListState,
}

impl Beacons {
    fn new(items: Vec<Zombie>) -> Beacons {
        Beacons {
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

fn context_menu(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<(),Box<dyn std::error::Error>>{
        let instructions = Spans::from(vec![Span::styled(
            "This is a stand in for the general instructions on how to make use of this application.",
            Style::default().fg(Color::Green),
        )]);
        let instructions_box = Paragraph::new(instructions)
            .block(
                Block::default()
            )
            .alignment(ratatui::layout::Alignment::Center)
            .wrap(Wrap { trim: true });
        let chunks = Layout::default().direction(Direction::Horizontal).constraints([Percentage(80), Percentage(20)]).split(terminal.size()?);
    terminal.draw(|f| {
        f.render_widget(instructions_box,chunks[0]);
    })?;
        std::thread::sleep(std::time::Duration::from_secs(5));
Ok(())
}

#[derive(Serialize, Deserialize, Debug, Clone,PartialEq)]
struct Zombie {
    ip: String,
    os: String,
    user: String,
}



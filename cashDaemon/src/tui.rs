use std::io;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::CrosstermBackend,
    Terminal,
    widgets::{Block, Borders},
    layout::{Layout, Constraint, Direction},
};


pub fn start_tui() -> Result<(), std::io::Error> {
    println!("Starting TUI");
    
    enable_raw_mode()?;

    let mut stdout = io::stdout();

    // Alternatvie Screen: Creates a fullscreen terminal ui
    // enables mouse capture
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    // set up crossterm backend
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // run the main event loop of the UI
    let res = run(&mut terminal);

    //cleanup: restore terminal to original Starte
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    res
}

fn run<B: tui::backend::Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        // Draw the UI on each iteration
        terminal.draw(|f| {
            let size = f.size();

            // create block widget with borders and a title
            let block = Block::default()
                .title("Rust tui with crossterm - press q to quit")
                .borders(Borders::ALL);

            //Render the block to fill the whole terminal
            f.render_widget(block, size);
        })?;

        // check for input events (like key press)
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()?{
                // Quit the app when the user press q
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }
    Ok(())
}














use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::Paragraph,
};
use std::io::{self, stdout};
use std::time::Duration;

struct App {
    should_quit: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut terminal = init_terminal()?;
    let mut app = App { should_quit: false };

    run(&mut terminal, &mut app).await?;

    restore_terminal()?;
    Ok(())
}

fn init_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;
    let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    Ok(terminal)
}

async fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> Result<()> {
    while !app.should_quit {
        terminal.draw(|frame| {
            let area = frame.area();
            frame.render_widget(
                Paragraph::new("Hello, Reqline! (Press 'q' to quit)")
                    .white()
                    .on_blue(),
                area,
            );
        })?;
        handle_events(app).await?;
    }
    Ok(())
}

async fn handle_events(app: &mut App) -> Result<()> {
    if event::poll(Duration::from_millis(250))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                app.should_quit = true;
            }
        }
    }
    Ok(())
}

fn restore_terminal() -> Result<()> {
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;
    Ok(())
}

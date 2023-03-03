use crossterm::{
    event::{poll, read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, time::Duration};
use tui::{
    backend::CrosstermBackend,
    layout::Alignment,
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|frame| {
            let size = frame.size();
            let block = Block::default()
                .title("Hello World!")
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Rgb(0x59, 0xf2, 0x70)));

            let p = Span::styled("Some text", Style::default().fg(Color::LightCyan));
            frame.render_widget(
                Paragraph::new(p).block(block).alignment(Alignment::Center),
                size,
            );
        })?;

        if poll(Duration::from_millis(10))? {
            match read()? {
                Event::Key(event) => {
                    if event.code == KeyCode::Char('q') {
                        break;
                    }
                }
                _ => continue,
            }
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

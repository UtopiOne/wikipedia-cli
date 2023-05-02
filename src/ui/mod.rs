use crossterm::{
    event::DisableMouseCapture,
    execute,
    terminal::{disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, thread, time::Duration};
use tui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, Paragraph, Wrap},
    Terminal,
};

pub fn render_terminal(text: String) -> Result<(), io::Error> {
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {
        let size = f.size();
        let paragraph = declare_paragraph(text);
        f.render_widget(paragraph, size);
    })?;

    thread::sleep(Duration::from_secs(3));

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

/// UI is created here.
pub fn declare_paragraph<'a>(text: String) -> Paragraph<'a> {
    //
    // Paragraph where the wiki text will be displayed.
    Paragraph::new(text)
        .block(Block::default().title("Title").borders(Borders::ALL))
        .alignment(tui::layout::Alignment::Left)
        .wrap(Wrap { trim: true })
}

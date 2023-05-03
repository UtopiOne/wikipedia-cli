use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, thread, time::Duration};
use tui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, Paragraph, Wrap},
    Terminal,
};

pub struct ArticleDisplay {
    pub title: String,
    pub contents: String,
}

impl ArticleDisplay {
    pub fn new(article: ArticleDisplay) -> Result<(), io::Error> {
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        terminal.draw(|f| {
            let size = f.size();
            let paragraph = declare_text(article.contents, article.title);
            f.render_widget(paragraph, size);
        })?;

        thread::sleep(Duration::from_secs(20));

        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        Ok(())
    }
}
/// UI is created here.
pub fn declare_text<'a>(text: String, title: String) -> Paragraph<'a> {
    //
    // Paragraph where the wiki text will be displayed.
    Paragraph::new(text)
        .block(Block::default().title(title).borders(Borders::ALL))
        .alignment(tui::layout::Alignment::Left)
        .wrap(Wrap { trim: true })
}

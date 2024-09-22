use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{fs, io, path::PathBuf};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Terminal,
};

extern crate crossterm;
extern crate tui;
struct App {
    current_path: PathBuf,
    entries: Vec<PathBuf>,
    selected: usize,
    preview: String,
}

impl App {
    fn new() -> io::Result<App> {
        let current_path = std::env::current_dir()?;
        let entries = fs::read_dir(&current_path)?
            .filter_map(|entry| entry.ok().map(|e| e.path()))
            .collect();
        Ok(App {
            current_path,
            entries,
            selected: 0,
            preview: String::new(),
        })
    }

    fn move_up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    fn move_down(&mut self) {
        if self.selected < self.entries.len() - 1 {
            self.selected += 1;
        }
    }

    fn enter_directory(&mut self) -> io::Result<()> {
        if let Some(selected_path) = self.entries.get(self.selected) {
            if selected_path.is_dir() {
                self.current_path = selected_path.clone();
                self.entries = fs::read_dir(&self.current_path)?
                    .filter_map(|entry| entry.ok().map(|e| e.path()))
                    .collect();
                self.selected = 0;
            }
        }
        Ok(())
    }

    fn go_up(&mut self) -> io::Result<()> {
        if let Some(parent) = self.current_path.parent() {
            self.current_path = parent.to_path_buf();
            self.entries = fs::read_dir(&self.current_path)?
                .filter_map(|entry| entry.ok().map(|e| e.path()))
                .collect();
            self.selected = 0;
        }
        Ok(())
    }

    fn update_preview(&mut self) -> io::Result<()> {
        self.preview.clear();
        if let Some(selected_path) = self.entries.get(self.selected) {
            if selected_path.is_file() {
                if let Ok(content) = fs::read_to_string(selected_path) {
                    self.preview = content.lines().take(20).collect::<Vec<_>>().join("\n");
                } else {
                    self.preview = "Unable to read file content.".to_string();
                }
            } else if selected_path.is_dir() {
                self.preview = "Directory selected. Press 'l' to enter.".to_string();
            }
        }
        Ok(())
    }
}

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new()?;
    let res = run_app(&mut terminal, app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: tui::backend::Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Min(0),
                        Constraint::Length(3),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            let current_dir = Span::raw(app.current_path.to_string_lossy());
            let header = Paragraph::new(current_dir)
                .style(Style::default().fg(Color::Yellow))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Current Directory"),
                );
            f.render_widget(header, chunks[0]);

            let horizontal_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(chunks[1]);

            let items: Vec<ListItem> = app
                .entries
                .iter()
                .map(|path| {
                    let name = path.file_name().unwrap().to_string_lossy();
                    if path.is_dir() {
                        ListItem::new(format!("/{}", name)).style(Style::default().fg(Color::Blue))
                    } else {
                        ListItem::new(name.to_string())
                    }
                })
                .collect();

            let mut list_state = ListState::default();
            list_state.select(Some(app.selected));

            let list = List::new(items)
                .block(Block::default().title("Files").borders(Borders::ALL))
                .highlight_style(
                    Style::default()
                        .bg(Color::DarkGray)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol("> ");

            f.render_stateful_widget(list, horizontal_chunks[0], &mut list_state);

            let preview = Paragraph::new(app.preview.as_str())
                .block(Block::default().title("Preview").borders(Borders::ALL))
                .wrap(Wrap { trim: true });
            f.render_widget(preview, horizontal_chunks[1]);

            let footer = Paragraph::new("q: Quit | j: Down | k: Up | l: Enter | h: Back")
                .style(Style::default().fg(Color::White))
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(footer, chunks[2]);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Char('j') => {
                    app.move_down();
                    app.update_preview()?;
                }
                KeyCode::Char('k') => {
                    app.move_up();
                    app.update_preview()?;
                }
                KeyCode::Char('l') => {
                    app.enter_directory()?;
                    app.update_preview()?;
                }
                KeyCode::Char('h') => {
                    app.go_up()?;
                    app.update_preview()?;
                }
                _ => {}
            }
        }
    }
}

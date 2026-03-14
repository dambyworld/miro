use std::io::{self, Stdout};

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Text};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph};

use crate::app::SessionManager;
use crate::model::{ProviderKind, SessionRecord};
use crate::run_resume_command;

pub fn run_tui(manager: SessionManager) -> Result<()> {
    let mut app = AppState::new(manager)?;
    let mut terminal = init_terminal()?;
    let result = run_event_loop(&mut terminal, &mut app);
    restore_terminal(&mut terminal)?;
    result
}

fn init_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    Ok(Terminal::new(backend)?)
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

fn run_event_loop(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    app: &mut AppState,
) -> Result<()> {
    loop {
        terminal.draw(|frame| app.draw(frame))?;
        let Event::Key(key) = event::read()? else {
            continue;
        };
        if key.kind != KeyEventKind::Press {
            continue;
        }

        if app.confirm_delete {
            match key.code {
                KeyCode::Char('y') => app.delete_selected()?,
                KeyCode::Esc | KeyCode::Char('n') => app.confirm_delete = false,
                _ => {}
            }
            continue;
        }

        if app.search_mode {
            match key.code {
                KeyCode::Esc => app.search_mode = false,
                KeyCode::Enter => app.search_mode = false,
                KeyCode::Backspace => {
                    app.query.pop();
                    app.reset_selection();
                }
                KeyCode::Char(character) => {
                    app.query.push(character);
                    app.reset_selection();
                }
                _ => {}
            }
            continue;
        }

        match key.code {
            KeyCode::Char('q') => break,
            KeyCode::Up => app.select_previous(),
            KeyCode::Down => app.select_next(),
            KeyCode::Char('/') => app.search_mode = true,
            KeyCode::Char('f') => app.cycle_provider_filter()?,
            KeyCode::Char('r') => app.reload()?,
            KeyCode::Char('d') => {
                if app.selected_session().is_some() {
                    app.confirm_delete = true;
                }
            }
            KeyCode::Enter => {
                if let Some(session) = app.selected_session() {
                    restore_terminal(terminal)?;
                    let result = run_resume_command(&session);
                    init_terminal_state_after_command(terminal)?;
                    result?;
                    app.status = Some("resumed session and returned to Miro".to_string());
                    app.reload()?;
                }
            }
            _ => {}
        }
    }
    Ok(())
}

fn init_terminal_state_after_command(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<()> {
    enable_raw_mode()?;
    execute!(terminal.backend_mut(), EnterAlternateScreen)?;
    Ok(())
}

struct AppState {
    manager: SessionManager,
    sessions: Vec<SessionRecord>,
    selected: usize,
    provider_filter: Option<ProviderKind>,
    query: String,
    search_mode: bool,
    confirm_delete: bool,
    status: Option<String>,
}

impl AppState {
    fn new(manager: SessionManager) -> Result<Self> {
        let sessions = manager.list_sessions(None)?;
        Ok(Self {
            manager,
            sessions,
            selected: 0,
            provider_filter: None,
            query: String::new(),
            search_mode: false,
            confirm_delete: false,
            status: None,
        })
    }

    fn reload(&mut self) -> Result<()> {
        self.sessions = self.manager.list_sessions(self.provider_filter)?;
        self.selected = self
            .selected
            .min(self.filtered_sessions().len().saturating_sub(1));
        Ok(())
    }

    fn cycle_provider_filter(&mut self) -> Result<()> {
        self.provider_filter = match self.provider_filter {
            None => Some(ProviderKind::Codex),
            Some(ProviderKind::Codex) => Some(ProviderKind::ClaudeCode),
            Some(ProviderKind::ClaudeCode) => None,
        };
        self.reload()?;
        self.reset_selection();
        Ok(())
    }

    fn reset_selection(&mut self) {
        self.selected = 0;
    }

    fn filtered_sessions(&self) -> Vec<SessionRecord> {
        let query = self.query.trim().to_lowercase();
        self.sessions
            .iter()
            .filter(|session| {
                if query.is_empty() {
                    return true;
                }
                session.title.to_lowercase().contains(&query)
                    || session
                        .preview
                        .as_deref()
                        .unwrap_or_default()
                        .to_lowercase()
                        .contains(&query)
                    || session
                        .cwd
                        .display()
                        .to_string()
                        .to_lowercase()
                        .contains(&query)
            })
            .cloned()
            .collect()
    }

    fn selected_session(&self) -> Option<SessionRecord> {
        self.filtered_sessions().get(self.selected).cloned()
    }

    fn select_next(&mut self) {
        let len = self.filtered_sessions().len();
        if len > 0 {
            self.selected = (self.selected + 1).min(len - 1);
        }
    }

    fn select_previous(&mut self) {
        self.selected = self.selected.saturating_sub(1);
    }

    fn delete_selected(&mut self) -> Result<()> {
        if let Some(session) = self.selected_session() {
            self.manager.delete_session(&session)?;
            self.status = Some(format!(
                "deleted {} {}",
                session.provider, session.session_id
            ));
            self.confirm_delete = false;
            self.reload()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut ratatui::Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(10),
                Constraint::Length(4),
            ])
            .split(frame.area());

        let filter_label = match self.provider_filter {
            None => "all".to_string(),
            Some(provider) => provider.to_string(),
        };
        let search_label = if self.search_mode {
            format!("search: {}", self.query)
        } else if self.query.is_empty() {
            "search: /".to_string()
        } else {
            format!("search: {}", self.query)
        };
        let header = Paragraph::new(format!(
            "Miro  filter: {}  sessions: {}  {}",
            filter_label,
            self.filtered_sessions().len(),
            search_label
        ))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Session Browser"),
        );
        frame.render_widget(header, chunks[0]);

        let filtered = self.filtered_sessions();
        let items: Vec<ListItem> = if filtered.is_empty() {
            vec![ListItem::new("No sessions found")]
        } else {
            filtered
                .iter()
                .map(|session| {
                    let preview = session.preview.as_deref().unwrap_or("-");
                    ListItem::new(Text::from(vec![
                        Line::from(format!("[{}] {}", session.provider.as_str(), session.title)),
                        Line::from(format!("  {}", preview)),
                        Line::from(format!(
                            "  {}  {}",
                            session.cwd.display(),
                            session.updated_at.format("%Y-%m-%d %H:%M")
                        )),
                    ]))
                })
                .collect()
        };

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Sessions"))
            .highlight_style(
                Style::default()
                    .bg(Color::Rgb(32, 58, 90))
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            );
        let mut state = ListState::default();
        if !filtered.is_empty() {
            state.select(Some(self.selected.min(filtered.len() - 1)));
        }
        frame.render_stateful_widget(list, chunks[1], &mut state);

        let status = self.status.as_deref().unwrap_or(
            "Up/Down move  Enter resume  d delete  f filter  / search  r refresh  q quit",
        );
        let footer =
            Paragraph::new(status).block(Block::default().borders(Borders::ALL).title("Help"));
        frame.render_widget(footer, chunks[2]);

        if self.confirm_delete {
            let area = centered_rect(60, 20, frame.area());
            frame.render_widget(Clear, area);
            let body = if let Some(session) = self.selected_session() {
                format!(
                    "Delete '{}'?\n\nPress y to confirm or n/Esc to cancel.",
                    session.title
                )
            } else {
                "Nothing selected".to_string()
            };
            let dialog = Paragraph::new(body)
                .style(Style::default().fg(Color::White).bg(Color::Rgb(70, 20, 20)))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Confirm Delete"),
                );
            frame.render_widget(dialog, area);
        }
    }
}

fn centered_rect(
    percent_x: u16,
    percent_y: u16,
    area: ratatui::layout::Rect,
) -> ratatui::layout::Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

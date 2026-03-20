use std::io::{self, Stdout};

use anyhow::Result;
use chrono::{DateTime, Local};
use crossterm::cursor::{Hide, Show};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{
    Clear as TerminalClear, ClearType, EnterAlternateScreen, LeaveAlternateScreen,
    disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph};

use crate::app::SessionManager;
use crate::config::MiroConfig;
use crate::model::{ProviderKind, SessionRecord};
use crate::theme::{Theme, ThemeName};
use crate::run_resume_command;

pub fn run_tui(manager: SessionManager, theme: Theme) -> Result<()> {
    let mut app = AppState::new(manager, theme)?;
    let mut terminal = init_terminal()?;
    let result = run_event_loop(&mut terminal, &mut app);
    restore_terminal(&mut terminal)?;
    result
}

fn init_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(
        stdout,
        EnterAlternateScreen,
        TerminalClear(ClearType::All),
        Hide
    )?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    Ok(terminal)
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), Show, LeaveAlternateScreen)?;
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

        if app.theme_menu_open {
            match key.code {
                KeyCode::Esc | KeyCode::Char('t') => app.close_theme_menu(),
                KeyCode::Up => app.select_previous_theme(),
                KeyCode::Down => app.select_next_theme(),
                KeyCode::Enter => app.apply_selected_theme(),
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
            KeyCode::Char('r') => app.refresh_with_feedback(),
            KeyCode::Char('t') => app.open_theme_menu(),
            KeyCode::Char('c') => app.copy_selected_session_id(),
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
                    match result {
                        Ok(()) => {
                            app.status = Some("resumed session and returned to Miro".to_string());
                            app.refresh_without_status()?;
                        }
                        Err(error) => {
                            app.status = Some(format!("resume failed: {error}"));
                        }
                    }
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
    execute!(
        terminal.backend_mut(),
        EnterAlternateScreen,
        TerminalClear(ClearType::All),
        Hide
    )?;
    terminal.clear()?;
    terminal.hide_cursor()?;
    terminal.autoresize()?;
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
    last_refreshed_at: Option<DateTime<Local>>,
    last_refresh_count: usize,
    theme: Theme,
    theme_menu_open: bool,
    theme_selected: usize,
}

impl AppState {
    fn new(manager: SessionManager, theme: Theme) -> Result<Self> {
        let sessions = manager.list_sessions(None)?;
        let theme_selected = ThemeName::all()
            .iter()
            .position(|candidate| *candidate == theme.id)
            .unwrap_or(0);
        let initial_count = sessions.len();
        let refreshed_at = Local::now();
        let mut app = Self {
            manager,
            sessions,
            selected: 0,
            provider_filter: None,
            query: String::new(),
            search_mode: false,
            confirm_delete: false,
            status: None,
            last_refreshed_at: None,
            last_refresh_count: 0,
            theme,
            theme_menu_open: false,
            theme_selected,
        };
        app.record_refresh(refreshed_at, initial_count);
        app.status = Some(format_refresh_status(initial_count, refreshed_at));
        Ok(app)
    }

    fn refresh_without_status(&mut self) -> Result<()> {
        self.sessions = self.manager.list_sessions(self.provider_filter)?;
        self.selected = self
            .selected
            .min(self.filtered_sessions().len().saturating_sub(1));
        Ok(())
    }

    fn refresh_with_feedback(&mut self) {
        match self.refresh_without_status() {
            Ok(()) => {
                let refreshed_at = Local::now();
                let visible_count = self.filtered_sessions().len();
                self.record_refresh(refreshed_at, visible_count);
                self.status = Some(format_refresh_status(visible_count, refreshed_at));
            }
            Err(error) => {
                self.status = Some(format!("refresh failed: {error}"));
            }
        }
    }

    fn cycle_provider_filter(&mut self) -> Result<()> {
        self.provider_filter = match self.provider_filter {
            None => Some(ProviderKind::Codex),
            Some(ProviderKind::Codex) => Some(ProviderKind::ClaudeCode),
            Some(ProviderKind::ClaudeCode) => None,
        };
        self.refresh_without_status()?;
        self.reset_selection();
        Ok(())
    }

    fn record_refresh(&mut self, refreshed_at: DateTime<Local>, visible_count: usize) {
        self.last_refreshed_at = Some(refreshed_at);
        self.last_refresh_count = visible_count;
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

    fn copy_selected_session_id(&mut self) {
        let Some(session) = self.selected_session() else {
            return;
        };
        use std::io::Write;
        use std::process::{Command, Stdio};
        let result = Command::new("pbcopy")
            .stdin(Stdio::piped())
            .spawn()
            .and_then(|mut child| {
                child
                    .stdin
                    .as_mut()
                    .unwrap()
                    .write_all(session.session_id.as_bytes())?;
                child.wait()
            });
        self.status = match result {
            Ok(_) => Some(format!("copied: {}", session.session_id)),
            Err(e) => Some(format!("copy failed: {e}")),
        };
    }

    fn delete_selected(&mut self) -> Result<()> {
        if let Some(session) = self.selected_session() {
            self.manager.delete_session(&session)?;
            self.status = Some(format!(
                "deleted {} {}",
                session.provider, session.session_id
            ));
            self.confirm_delete = false;
            self.refresh_without_status()?;
        }
        Ok(())
    }

    fn open_theme_menu(&mut self) {
        self.theme_menu_open = true;
        self.theme_selected = ThemeName::all()
            .iter()
            .position(|candidate| *candidate == self.theme.id)
            .unwrap_or(0);
    }

    fn close_theme_menu(&mut self) {
        self.theme_menu_open = false;
    }

    fn select_next_theme(&mut self) {
        let len = ThemeName::all().len();
        if len > 0 {
            self.theme_selected = (self.theme_selected + 1).min(len - 1);
        }
    }

    fn select_previous_theme(&mut self) {
        self.theme_selected = self.theme_selected.saturating_sub(1);
    }

    fn apply_selected_theme(&mut self) {
        let selected_theme = ThemeName::all()
            .get(self.theme_selected)
            .copied()
            .unwrap_or(ThemeName::TomorrowNightBlue);
        self.theme = Theme::get(selected_theme);
        self.theme_menu_open = false;
        self.status = Some(format!("theme changed to {}", selected_theme.display_name()));
        MiroConfig::save_theme(selected_theme);
    }

    fn draw(&self, frame: &mut ratatui::Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(4),
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
        let refreshed_label = self
            .last_refreshed_at
            .map(format_refresh_time)
            .unwrap_or_else(|| "-".to_string());
        let selected_id_label = self
            .selected_session()
            .map(|s| {
                let short = &s.session_id[..s.session_id.len().min(8)];
                format!("id:{short}...")
            })
            .unwrap_or_else(|| "id:-".to_string());
        let header = Paragraph::new(Text::from(vec![
            Line::from(vec![
                Span::raw(" MIRO  "),
                Span::raw(format!("theme:{}  ", self.theme.id.display_name())),
                Span::raw(format!("filter:{}  ", filter_label)),
                Span::raw(format!("sessions:{}  ", self.filtered_sessions().len())),
                Span::raw(selected_id_label),
            ]),
            Line::from(vec![
                Span::raw(format!(" refreshed:{}  ", refreshed_label)),
                Span::raw(" search:"),
                Span::raw(compact_search_label(&search_label)),
            ]),
        ]))
        .style(self.theme.header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Browser")
                .border_style(self.theme.header_border),
        );
        frame.render_widget(header, chunks[0]);

        let filtered = self.filtered_sessions();
        let items: Vec<ListItem> = if filtered.is_empty() {
            vec![ListItem::new(Line::from(vec![Span::styled(
                "No sessions found",
                self.theme.empty_state,
            )]))]
        } else {
            filtered
                .iter()
                .map(|session| {
                    let preview = session.preview.as_deref().unwrap_or("-");
                    let provider_style = match session.provider {
                        ProviderKind::Codex => self.theme.codex_badge,
                        ProviderKind::ClaudeCode => self.theme.claude_badge,
                    };

                    ListItem::new(Line::from(vec![
                        Span::styled(format!("{: <7}", session.provider.as_str()), provider_style),
                        Span::raw(" "),
                        Span::styled(session.title.clone(), self.theme.title),
                        Span::raw("  "),
                        Span::styled(preview.to_string(), self.theme.preview),
                        Span::raw("  "),
                        Span::styled(
                            format!(
                                "{}  {}",
                                session.cwd.display(),
                                session.updated_at.format("%m-%d %H:%M")
                            ),
                            self.theme.meta,
                        ),
                    ]))
                })
                .collect()
        };

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Sessions")
                    .border_style(self.theme.list_border),
            )
            .highlight_style(self.theme.selected_row)
            .style(self.theme.app_background);
        let mut state = ListState::default();
        if !filtered.is_empty() {
            state.select(Some(self.selected.min(filtered.len() - 1)));
        }
        frame.render_stateful_widget(list, chunks[1], &mut state);

        let help_text =
            " Up/Down move  Enter resume  c copy-id  t theme  d delete  f filter  r refresh  q quit ";
        let status_text = self.status.as_deref().unwrap_or(" ready ");
        let footer = Paragraph::new(Text::from(vec![
            Line::from(Span::styled(help_text, self.theme.footer_hint)),
            Line::from(vec![
                Span::styled(" / search  ", self.theme.footer_hint),
                Span::styled(format!("| {}", status_text), self.theme.footer_status),
            ]),
        ]))
        .style(self.theme.footer)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(self.theme.list_border),
        );
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
                .style(self.theme.dialog)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Confirm Delete")
                        .border_style(self.theme.dialog_border),
                );
            frame.render_widget(dialog, area);
        }

        if self.theme_menu_open {
            let area = centered_rect(68, 42, frame.area());
            frame.render_widget(Clear, area);
            let theme_items: Vec<ListItem> = ThemeName::all()
                .iter()
                .map(|theme| {
                    let suffix = if *theme == ThemeName::TomorrowNightBlue {
                        " (default)"
                    } else {
                        ""
                    };
                    ListItem::new(Line::from(vec![
                        Span::styled(
                            format!("{}{}", theme.display_name(), suffix),
                            self.theme.title,
                        ),
                        Span::raw("  "),
                        Span::styled(theme.description(), self.theme.meta),
                    ]))
                })
                .collect();

            let theme_list = List::new(theme_items)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Theme Picker")
                        .border_style(self.theme.header_border),
                )
                .highlight_style(self.theme.selected_row)
                .style(self.theme.app_background);
            let mut theme_state = ListState::default();
            theme_state.select(Some(self.theme_selected));
            frame.render_stateful_widget(theme_list, area, &mut theme_state);
        }
    }
}

fn format_refresh_time(refreshed_at: DateTime<Local>) -> String {
    refreshed_at.format("%H:%M:%S").to_string()
}

fn format_refresh_status(visible_count: usize, refreshed_at: DateTime<Local>) -> String {
    format!(
        "refreshed {} sessions at {}",
        visible_count,
        format_refresh_time(refreshed_at)
    )
}

fn compact_search_label(search_label: &str) -> String {
    search_label
        .strip_prefix("search: ")
        .unwrap_or(search_label)
        .to_string()
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

#[cfg(test)]
mod tests {
    use chrono::{Local, TimeZone};

    use super::{compact_search_label, format_refresh_status, format_refresh_time};

    #[test]
    fn formats_refresh_time_for_header() {
        let refreshed_at = Local.with_ymd_and_hms(2026, 3, 16, 13, 25, 8).unwrap();
        assert_eq!(format_refresh_time(refreshed_at), "13:25:08");
    }

    #[test]
    fn formats_refresh_status_for_footer() {
        let refreshed_at = Local.with_ymd_and_hms(2026, 3, 16, 13, 25, 8).unwrap();
        assert_eq!(
            format_refresh_status(24, refreshed_at),
            "refreshed 24 sessions at 13:25:08"
        );
    }

    #[test]
    fn compacts_search_label_for_second_header_row() {
        assert_eq!(compact_search_label("search: /"), "/");
        assert_eq!(compact_search_label("search: codex"), "codex");
    }
}

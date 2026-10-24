use ratatui::{
    layout::{Layout, Alignment, Constraint, Direction},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame
};
use ratatui::layout::Constraint::Percentage;
use crate::app::{App, Screen};
use crate::ascii::models::LOGO;

/// Main rendering function
pub fn render(frame: &mut Frame, app: &mut App) {
    match app.screen {
        Screen::MainMenu => render_main_menu(frame, app),
        Screen::Workbench => render_workbench(frame, app),
    }
}

/// Rendering function for workbench
pub fn render_workbench(frame: &mut Frame, app: &mut App) {
    // Separation of view ->> main frame vs log
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(10), Constraint::Length(6)])
        .split(frame.area())
    ;

    // Separation of main frame ->> working bench vs tools column
    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Percentage(70), Percentage(30)])
        .split(main_chunks[0])
    ;

    // 1. Working bench
    let ascii_art = app.current_watch.get_art_for_state(app.watch_state);
    let status_line = format!("\n\n{}", app.watch_state.status_text());
    let combined_display = format!("{}{}", ascii_art, status_line);

    let workbench = Paragraph::new(combined_display)
        .block(Block::default().borders(Borders::ALL).title("Workbench"))
        .alignment(Alignment::Center)
    ;

    frame.render_widget(workbench, top_chunks[0]);

    // 2. Case with tools
    let tools: Vec<ListItem> = app.tools
        .iter()
        .map(|tool| ListItem::new(tool.name()))
        .collect()
    ;

    let tools_list = List::new(tools)
        .block(Block::default().borders(Borders::ALL).title("Tools"))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        )
        .highlight_symbol(">> ")
    ;

    frame.render_stateful_widget(
        tools_list,
        top_chunks[1],
        &mut app.tools_state
    );

    // 3. ->> Logs of all action
    let recent_logs: Vec<ListItem> = app.log_messages
        .iter()
        .rev()
        .take(4)
        .rev()
        .map(|msg| ListItem::new(msg.as_str()))
        .collect()
    ;

    let log_box = List::new(recent_logs)
        .block(Block::default().borders(Borders::ALL).title("Logs"))
    ;

    frame.render_widget(log_box, main_chunks[1]);
}

pub fn render_main_menu(frame: &mut Frame, app: &mut App) {
    // Vertical split into sections
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Percentage(20), // Odsazení shora
            Constraint::Length(8), // Height of logo
            Constraint::Length(6), // Height of menu
            Constraint::Min(2), // Footer with hint
        ])
        .split(frame.area());

    // ASCII logo of the game
    let logo = Paragraph::new(LOGO)
        .alignment(Alignment::Center)
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        );
    frame.render_widget(logo, chunks[1]);

    // Options in menu
    let options: Vec<ListItem> = app
        .menu_options
        .iter()
        .map(|item| ListItem::new(format!("  {}  ", item.label())))
        .collect();

    let menu_list = List::new(options)
        .block(Block::default().borders(Borders::NONE))
        .highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▶ ");

    // Horizontal align of menu (30 % uprostřed)
    let menu_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(35),
            Constraint::Percentage(30),
            Constraint::Percentage(35),
        ])
        .split(chunks[2])[1];

    frame.render_stateful_widget(menu_list, menu_area, &mut app.menu_state);

    // Hint for player
    let footer = Paragraph::new("Press [UP/DOWN] to navigate • [ENTER] to select • [Q] to quit")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::DarkGray));
    frame.render_widget(footer, chunks[3]);
}
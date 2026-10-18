use ratatui::{
    layout::{Layout, Alignment, Constraint, Direction},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame
};
use ratatui::layout::Constraint::Percentage;
use crate::app::App;

/// Main rendering function
pub fn render(frame: &mut Frame, app: &mut App) {
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
    let workbench_text = vec![
        "",
        "          .---.          ",
        "         /     \\         ",
        "        |  (O)  |        <- (Seiko NH35)",
        "         \\     /         ",
        "          '---'          ",
        "",
        "  State: Closed",
    ];

    let workbench = Paragraph::new(workbench_text.join("\n"))
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
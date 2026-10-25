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
        Screen::Catalog => render_catalog_menu(frame, app),
        Screen::Parts => render_parts_menu(frame, app),
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

pub fn render_catalog_menu(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(10), Constraint::Length(3)])
        .split(frame.area())
    ;

    // List of models (25 %), Image (35 %), Lore (40 %)
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(35),
            Constraint::Percentage(40),
        ])
        .split(chunks[0])
    ;

    // List of models
    let model_items: Vec<ListItem> = app
        .catalog_models
        .iter()
        .map(|w| ListItem::new(format!("{} - {}", w.brand, w.name)))
        .collect()
    ;

    let model_list = List::new(model_items)
        .block(Block::default().borders(Borders::ALL).title(" Models archive "))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ")
    ;

    frame.render_stateful_widget(model_list, cols[0], &mut app.catalog_state);

    // Currently selected model for detail
    let selected_index = app.catalog_state.selected().unwrap_or(0);
    let watch = app.catalog_models[selected_index];

    // Full watch image
    let watch_image = Paragraph::new(watch.full_watch)
        .block(Block::default().borders(Borders::ALL).title(" Visual Blueprint "))
        .alignment(Alignment::Center)
    ;

    frame.render_widget(watch_image, cols[1]);

    // Logo, specification, lore
    let info_text = format!(
        "--- {} ---\n\
        \"{}\"\n\n\
        • Model: {}\n\
        • Movement: {}\n\
        • Water Resistance: {}\n\
        • Construction: {}\n\n\
        [LORE & DOSSIER]:\n\
        {}",
        watch.brand,
        watch.tagline,
        watch.name,
        watch.movement_spec,
        watch.water_resistance,
        watch.case_material,
        watch.lore_description
    );

    let details = Paragraph::new(info_text)
        .block(Block::default().borders(Borders::ALL).title(" Horological Dossier "))
        .style(Style::default().fg(Color::White))
        .wrap(ratatui::widgets::Wrap { trim: true })
    ;

    frame.render_widget(details, cols[2]);

    // Footer hint
    let footer = Paragraph::new(
        "▲/▼ Navigate Models • [ENTER] Take to Workbench • [ESC] Return to Menu", )
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::DarkGray))
    ;

    frame.render_widget(footer, chunks[1]);
}

/// Renders parts glossary
pub fn render_parts_menu(frame: &mut Frame, app: &mut App) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(10), Constraint::Length(3)])
        .split(frame.area())
    ;

    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(35), Constraint::Percentage(65)])
        .split(main_layout[0])
    ;

    // List of parts on the left
    let items: Vec<ListItem> = app
        .part_models
        .iter()
        .map(|part| ListItem::new(format!("• {}", part.name)))
        .collect()
    ;

    let parts_list_widget = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Components Index "),
        )
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ")
    ;

    frame.render_stateful_widget(parts_list_widget, cols[0], &mut app.part_state);

    // Technical detail
    let selected_index = app.part_state.selected().unwrap_or(0);
    let part = app.part_models[selected_index];

    let detail_text = format!(
        "=== {} ===\n\
        Category: {}\n\n\
        [FUNCTION & PURPOSE]\n\
        {}\n\n\
        [COMMON FAILURE MODES]\n\
        {}\n\n\
        [HOROLOGIST'S NOTE]\n\
        {}",
        part.name,
        part.category,
        part.function,
        part.failure_mode,
        part.lore_fact
    );

    let detail_widget = Paragraph::new(detail_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Technical Specification "),
        )
        .style(Style::default().fg(Color::White))
        .wrap(ratatui::widgets::Wrap { trim: true })
    ;

    frame.render_widget(detail_widget, cols[1]);

    // Footer hint
    let footer = Paragraph::new("▲/▼ Navigate Parts • [ESC] Return to Main Menu")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::DarkGray))
    ;

    frame.render_widget(footer, main_layout[1]);
}
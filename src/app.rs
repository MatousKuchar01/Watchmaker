use ratatui::widgets::ListState;

/// Available tools in workshop
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tool {
    Tweezers, // for parts handling
    CaseOpener, // for opening case
    HandPress, // for settling the glass and hands
    Loupe // for magnification
}

impl Tool {
    /// Returns UI name for tool
    pub fn name(&self) -> &'static str {
        match self {
            Tool::Tweezers => "Tweezers",
            Tool::CaseOpener => "Case Opener",
            Tool::HandPress => "Hand Press",
            Tool::Loupe => "Loupe",
        }
    }
}

/// Main struct that hold state of the whole app
pub struct App {
    /// Should app quit?
    pub should_quit: bool,
    /// List of all tools player can use
    pub tools: Vec<Tool>,
    /// State of selection in TUI list
    pub tools_state: ListState,
    /// History of log messages
    pub log_messages: Vec<String>,
}

impl App {
    /// Constructor - returns new instance of game in default state
    pub fn new() -> Self {
        let mut tools_state = ListState::default();
        tools_state.select(Some(0)); // on startup pick first tool in vec

        Self {
            should_quit: false,
            tools: vec![
                Tool::Tweezers,
                Tool::CaseOpener,
                Tool::HandPress,
                Tool::Loupe,
            ],
            tools_state,
            log_messages: vec![
                "You stepped into the watch workshop.".to_string(),
                "Use ARROWS to pick tool, ENTER to use, Q to quit.".to_string(),
            ],
        }
    }

    /// Moves selection of a tool down one index
    pub fn select_next_tool(&mut self) {
        if let Some(selection) = self.tools_state.selected() {
            let next = (selection + 1) % self.tools.len();
            self.tools_state.select(Some(next));
        }
    }

    /// Moves selection of a tool up one index
    pub fn select_previous_tool(&mut self) {
        if let Some(selection) = self.tools_state.selected() {
            let previous = if selection == 0 {
                self.tools.len() - 1
            } else {
                selection - 1
            };

            self.tools_state.select(Some(previous));
        }
    }

    /// Simulation of using the selected tool
    pub fn use_selected_tool(&mut self) {
        if let Some(selection) = self.tools_state.selected() {
            let tool = self.tools[selection];
            self.log_messages.push(format!("You used: {}", tool.name()));
        }
    }

    /// Quitting the game
    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}


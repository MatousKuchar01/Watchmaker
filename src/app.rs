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
            tools: vec![], //todo fill tools
            tools_state,
            log_messages: vec![], //todo fill messages
        }
    }

    /// Moves selection of a tool down one index
    pub fn select_next_tool(&mut self) {}

    /// Moves selection of a tool up one index
    pub fn select_previous_tool(&mut self) {}

    /// Simulation of using the selected tool
    pub fn use_selected_tool(&mut self) {}

    /// Quitting the game
    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}


use ratatui::widgets::ListState;
use crate::app::MainMenuOption::{
    Quit, StartWorkbench, WatchCatalog, Parts
};
use crate::app::Screen::MainMenu;
use crate::ascii::{models, WatchAscii};
use crate::parts::{parts, Part};

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

/// State of watch disassembly
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum WatchState {
    ClosedCase, // Sealed case back
    OpenCase, // Case back unscrewed (movement visible)
    MovementExtracted, // Movement sitting on the workbench pad
    HandsRemoved, // Hands detached from the pinion
}

impl WatchState {
    /// Helper text for UI
    pub fn status_text(&self) -> &'static str {
        match self {
            WatchState::ClosedCase => "Case: Sealed (screw-down case back locked)",
            WatchState::OpenCase => "Case: Open (Seiko NH35 movement visible)",
            WatchState::MovementExtracted => "Status: Movement extracted onto workbench pad",
            WatchState::HandsRemoved => "Status: Hands removed, bare dial exposed",
        }
    }
}

/// Enum for game screens
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    MainMenu,
    Workbench,
    Catalog,
    Parts
}

/// Choices in main menu
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MainMenuOption {
    StartWorkbench,
    WatchCatalog,
    Parts,
    Quit,
}

impl MainMenuOption {
    /// Label for menu option
    pub fn label(&self) -> &'static str {
        match self {
            StartWorkbench => "Enter Workshop",
            WatchCatalog => "Watch Catalog",
            Parts => "Parts glossary",
            Quit => "Quit Application",
        }
    }
}

/// Main struct that hold state of the whole app
pub struct App {
    /// Should app quit?
    pub should_quit: bool,
    /// Current screen
    pub screen: Screen,
    /// Options in the menu
    pub menu_options: Vec<MainMenuOption>,
    /// State of selection in menu
    pub menu_state: ListState,
    /// Catalog models
    pub catalog_models: &'static[&'static WatchAscii],
    /// State of selection in catalog
    pub catalog_state: ListState,
    /// Parts glossary
    pub part_models: &'static[&'static Part],
    /// State of selection in parts glossary
    pub part_state: ListState,
    /// List of all tools player can use
    pub tools: Vec<Tool>,
    /// State of selection in TUI list
    pub tools_state: ListState,
    /// State of watch
    pub watch_state: WatchState,
    /// Current watch
    pub current_watch: &'static WatchAscii,
    /// History of log messages
    pub log_messages: Vec<String>,
}

impl App {
    /// Constructor - returns new instance of game in default state
    pub fn new() -> Self {
        let mut tools_state = ListState::default();
        tools_state.select(Some(0)); // on startup pick first tool in vec

        let mut menu_state = ListState::default();
        menu_state.select(Some(0));

        let mut catalog_state = ListState::default();
        catalog_state.select(Some(0));

        let mut part_state = ListState::default();
        part_state.select(Some(0));

        Self {
            should_quit: false,
            screen: MainMenu,
            menu_options: vec![
                StartWorkbench,
                WatchCatalog,
                Parts,
                Quit
            ],
            menu_state,
            catalog_models: models::ALL_MODELS,
            catalog_state,
            part_models: parts::ALL_PARTS,
            part_state,
            tools: vec![
                Tool::Tweezers,
                Tool::CaseOpener,
                Tool::HandPress,
                Tool::Loupe,
            ],
            tools_state,
            watch_state: WatchState::ClosedCase,
            current_watch: &models::OREX_SUBAQUATIC,
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

            match (self.watch_state, tool) {
                // Step 1: Opening the case
                (WatchState::ClosedCase, Tool::CaseOpener) => {
                    self.watch_state = WatchState::OpenCase;
                    self.log_messages.push(
                        "SUCCESS: Case opener locked into notches and unscrewed the case back. NH35 movement exposed."
                            .to_string(),
                    );
                }
                (WatchState::ClosedCase, Tool::Tweezers) => {
                    self.log_messages.push(
                        "FAIL: Tweezers cannot grip the steel case back - you'll only bend the tips! Use the Case Opener."
                            .to_string(),
                    );
                }
                (WatchState::ClosedCase, _) => {
                    self.log_messages.push(
                        "This tool cannot be used on a sealed case back.".to_string(),
                    );
                }

                // Step 2: Extracting the movement
                (WatchState::OpenCase, Tool::Tweezers) => {
                    self.watch_state = WatchState::MovementExtracted;
                    self.log_messages.push(
                        "SUCCESS: Carefully gripped the spacer ring with tweezers and transferred the movement to the pad."
                            .to_string(),
                    );
                }
                (WatchState::OpenCase, Tool::CaseOpener) => {
                    self.log_messages.push(
                        "The case back is already removed. The case opener is no longer needed here."
                            .to_string(),
                    );
                }
                (WatchState::OpenCase, _) => {
                    self.log_messages.push(
                        "Handling the bare movement requires precision Tweezers.".to_string(),
                    );
                }

                // Step 3: Inspecting extracted movement
                (WatchState::MovementExtracted, Tool::Loupe) => {
                    self.log_messages.push(
                        "INSPECT: Under 10x magnification, the hour, minute, and second hands are firmly set on the pinion."
                            .to_string(),
                    );
                }

                (WatchState::MovementExtracted, Tool::Tweezers) => {
                    self.watch_state = WatchState::HandsRemoved;
                    self.log_messages.push(
                        "SUCCESS: With steady hands and fine tweezers, you lifted all three hands off the central pinion."
                            .to_string(),
                    );
                }

                // Default fallback
                _ => {
                    self.log_messages.push(format!(
                        "Tool '{}' has no effect at this stage of disassembly.",
                        tool.name()
                    ));
                }
            }
        }
    }

    /// Moves selection in menu
    pub fn menu_next(&mut self) {
        if let Some(i) = self.menu_state.selected() {
            let next = (i + 1) % self.menu_options.len();
            self.menu_state.select(Some(next));
        }
    }

    /// Moves selection in menu
    pub fn menu_previous(&mut self) {
        if let Some(i) = self.menu_state.selected() {
            let prev = if i == 0 { self.menu_options.len() - 1 } else { i - 1 };
            self.menu_state.select(Some(prev));
        }
    }

    pub fn menu_select(&mut self) {
        if let Some(i) = self.menu_state.selected() {
            match self.menu_options[i] {
                StartWorkbench => {
                    self.screen = Screen::Workbench; // to the game
                }
                WatchCatalog => {
                    self.screen = Screen::Catalog;
                }
                Parts => {
                    self.screen = Screen::Parts;
                }
                Quit => {
                    self.should_quit = true;
                }
            }
        }
    }

    /// Moves selection in catalog
    pub fn catalog_next(&mut self) {
        if let Some(i) = self.catalog_state.selected() {
            let next = (i + 1) % self.catalog_models.len();
            self.catalog_state.select(Some(next));
        }
    }

    /// Moves selection in catalog
    pub fn catalog_previous(&mut self) {
        if let Some(i) = self.catalog_state.selected() {
            let prev = if i == 0 { self.catalog_models.len() - 1 } else { i - 1 };
            self.catalog_state.select(Some(prev));
        }
    }

    /// Moves selection in Glossary
    pub fn parts_next(&mut self) {
        if let Some(i) = self.part_state.selected() {
            let next = (i + 1) % self.part_models.len();
            self.part_state.select(Some(next));
        }
    }

    /// Moves selection in Glossary
    pub fn parts_previous(&mut self) {
        if let Some(i) = self.part_state.selected() {
            let prev = if i == 0 { self.part_models.len() - 1 } else { i - 1 };
            self.part_state.select(Some(prev));
        }
    }

    /// Selects currently viewed watch as active for workshop model and jumps into Workbench!
    pub fn catalog_select_current(&mut self) {
        if let Some(i) = self.catalog_state.selected() {
            self.current_watch = self.catalog_models[i];
            self.watch_state = WatchState::ClosedCase;
            self.log_messages.push(format!("Now working on: {}", self.current_watch.name));
            self.screen = Screen::Workbench;
        }
    }

    /// Quitting the game
    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}


use crate::app::WatchState;

/// All watch ascii variants
#[derive(Clone, Copy)]
pub struct WatchAscii {
    pub case_back: &'static str,
    pub movement_cased: &'static str,
    pub movement_extracted: &'static str,
    pub dial_no_hands: &'static str,
}

impl WatchAscii {
    /// Returns ascii art based on actual state of watch
    pub fn get_art_for_state(&self, state: WatchState) -> &'static str {
        match state {
            WatchState::ClosedCase => self.case_back,
            WatchState::OpenCase => self.movement_cased,
            WatchState::HandsRemoved => self.dial_no_hands,
            WatchState::MovementExtracted => self.movement_extracted,
        }
    }
}

/// All watch models
pub mod models {
    use super::*;

    pub const LOGO: &'static str = include_str!("../ascii/logos/logo.txt");

    /*pub const SEAWARRIOR_300: WatchAscii = WatchAscii {
        case_back: include_str!("../ascii/watches/seawarrior_300/case_back.txt"),
        movement_cased: include_str!("../ascii/watches/seawarrior_300/movement_cased.txt"),
        movement_extracted: include_str!("../ascii/watches/seawarrior_300/movement_extracted.txt"),
        dial_no_hands: include_str!("../ascii/watches/seawarrior_300/dial_no_hands.txt"),
    };*/

    pub const OREX_SUBAQUATIC: WatchAscii = WatchAscii {
        case_back: include_str!("../ascii/watches/orex_subaquatic/case_back.txt"),
        movement_cased: include_str!("../ascii/watches/orex_subaquatic/movement_cased.txt"),
        movement_extracted: include_str!("../ascii/watches/orex_subaquatic/movement_extracted.txt"),
        dial_no_hands: include_str!("../ascii/watches/orex_subaquatic/dial_no_hands.txt"),
    };

    /*pub const ALPHA_ORBITMASTER: WatchAscii = WatchAscii {
        case_back: include_str!("../ascii/watches/alpha_orbitmaster/case_back.txt"),
        movement_cased: include_str!("../ascii/watches/alpha_orbitmaster/movement_cased.txt"),
        movement_extracted: include_str!("../ascii/watches/alpha_orbitmaster/movement_extracted.txt"),
        dial_no_hands: include_str!("../ascii/watches/alpha_orbitmaster/dial_no_hands.txt"),
    };*/
}
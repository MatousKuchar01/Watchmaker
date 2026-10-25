use crate::app::WatchState;

/// All watch ascii variants
#[derive(Clone, Copy)]
pub struct WatchAscii {
    // base info
    pub name: &'static str,
    pub brand: &'static str,
    pub tagline: &'static str,
    pub movement_spec: &'static str,
    pub water_resistance: &'static str,
    pub case_material: &'static str,
    pub lore_description: &'static str,

    // ascii arts
    pub full_watch: &'static str,
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
    name: "Sea-Warrior 300 Pro",
        brand: "XODA",
        tagline: "Worn by Secret Agents",
        movement_spec: "Co-Axial Frictionless 8800",
        water_resistance: "300 m",
        case_material: "Titanium & Ceramic",
        lore_description:
            "Equipped with laser-etched wave dial patterns and a helium escape \
            valve specifically added so owners can spend 20 minutes explaining \
            what it does at parties.",

        full_watch: include_str!("../ascii/watches/seawarrior_300/full_watch.txt"),
        case_back: include_str!("../ascii/watches/seawarrior_300/case_back.txt"),
        movement_cased: include_str!("../ascii/watches/seawarrior_300/movement_cased.txt"),
        movement_extracted: include_str!("../ascii/watches/seawarrior_300/movement_extracted.txt"),
        dial_no_hands: include_str!("../ascii/watches/seawarrior_300/dial_no_hands.txt"),
    };*/

    pub const OREX_SUBAQUATIC: WatchAscii = WatchAscii {
        name: "Sub-Aquatic 300",
        brand: "OREX GENÈVE",
        tagline: "The Choice of Hedge Fund Divers",
        movement_spec: "Calibre 3135-X (Automatic, 28,800 vph)",
        water_resistance: "300 m",
        case_material: "904L \"OysterFlex-Steel\"",
        lore_description:
        "Originally designed for deep-sea exploration, but 99.4% of units \
            never dive deeper than a corporate swimming pool. Comes with a \
            magnified cyclops lens to better read just how late you are.",

        full_watch: include_str!("../ascii/watches/orex_subaquatic/full_watch.txt"),
        case_back: include_str!("../ascii/watches/orex_subaquatic/case_back.txt"),
        movement_cased: include_str!("../ascii/watches/orex_subaquatic/movement_cased.txt"),
        movement_extracted: include_str!("../ascii/watches/orex_subaquatic/movement_extracted.txt"),
        dial_no_hands: include_str!("../ascii/watches/orex_subaquatic/dial_no_hands.txt"),
    };

    /*pub const ALPHA_ORBITMASTER: WatchAscii = WatchAscii {
    name: "Orbitmaster Moon Edition",
        brand: "ALPHA CHRONO",
        tagline: "The First Watch on the Moon",
        movement_spec: "Calibre 1861 (Manual-Wind, 48h Reserve)",
        water_resistance: "50 m",
        case_material: "Brushed Steel & Hesalite Glass",
        lore_description:
            "The quintessential space chronograph. If you drop it in the sink, \
            it might drown, but it will survive a vacuum. Requires daily \
            winding because batteries are for quitters.",

        full_watch: include_str!("../ascii/watches/alpha_orbitmaster/full_watch.txt"),
        case_back: include_str!("../ascii/watches/alpha_orbitmaster/case_back.txt"),
        movement_cased: include_str!("../ascii/watches/alpha_orbitmaster/movement_cased.txt"),
        movement_extracted: include_str!("../ascii/watches/alpha_orbitmaster/movement_extracted.txt"),
        dial_no_hands: include_str!("../ascii/watches/alpha_orbitmaster/dial_no_hands.txt"),
    };*/

    pub const ALL_MODELS: &[&'static WatchAscii] = &[
        //&SEAWARRIOR_300,
        &OREX_SUBAQUATIC,
        //&ALPHA_ORBITMASTER,
    ];
}
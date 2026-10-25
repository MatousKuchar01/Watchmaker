/// Information about a watch part
#[derive(Debug, Clone, Copy)]
pub struct Part {
    pub name: &'static str,
    pub category: &'static str,
    pub function: &'static str,
    pub failure_mode: &'static str,
    pub lore_fact: &'static str,
}

pub mod parts {
    use super::*;

    pub const CASE_BACK: Part = Part {
        name: "Case Back",
        category: "Exterior / Case",
        function: "Seals the movement inside the case, protects against dust, moisture, and greasy fingers.",
        failure_mode: "Stripped threads, cross-threading, gouged case back notches from cheap screwdrivers.",
        lore_fact: "Display case backs let you admire a decorated movement. Solid case backs hide the fact that you paid 5,000 EUR for a stock movement.",
    };

    pub const WINDING_ROTOR: Part = Part {
        name: "Winding Rotor",
        category: "Movement / Automatic",
        function: "Heavy swinging weight mounted on a ball bearing. Spins with wrist movement to wind the mainspring via kinetic energy.",
        failure_mode: "Worn ball bearings causing the rotor to scrape against the bridges or inside the case.",
        lore_fact: "If you shake your wrist next to your ear and it sounds like a bag of marbles, your rotor bearings are shot.",
    };

    pub const MAINSPRING: Part = Part {
        name: "Mainspring & Barrel",
        category: "Power Source",
        function: "Coiled spring-steel strip inside a toothed brass drum. Stores potential mechanical energy (the watch's 'battery').",
        failure_mode: "Spring fatigue (short power reserve), grease hardening, or catastrophic spring snap when hand-wound like a lawnmower.",
        lore_fact: "An unbroken mainspring under tension contains enough stored energy to launch itself clear across your room into an unreachable dimension.",
    };

    pub const BALANCE_WHEEL_HAIRSPRING: Part = Part {
        name: "Balance Wheel & Hairspring",
        category: "Regulator / Heartbeat",
        function: "The oscillating organ. Bounces back and forth at 21,600 to 28,800 beats per hour to divide time into precise fractions.",
        failure_mode: "Magnetization (watch runs hours fast), tangled hairspring turns, or bent pivots from dropping the watch on tile.",
        lore_fact: "The hairspring is finer than a human hair and made of high-tech alloys (Nivarox, silicon) so temperature swings don't make you 5 minutes late to brunch.",

    };

    pub const ESCAPEMENT_PALLET_FORK: Part = Part {
        name: "Escapement & Pallet Fork",
        category: "Escapement",
        function: "Translates the rotational energy of gear wheels into rhythmic impulse pushes to keep the balance wheel swinging.",
        failure_mode: "Dry synthetic ruby pallets, broken balance impulse pin, chipped teeth.",
        lore_fact: "This produces the iconic 'tick-tock' sound. Two synthetic ruby stones catch and release the escape wheel thousands of times an hour.",
    };

    pub const CROWN_STEM: Part = Part {
        name: "Crown & Stem",
        category: "Controls",
        function: "Interlocks with the keyless works to switch modes between manual winding, quick-setting the date, and setting the hands.",
        failure_mode: "Snapping the stem clean off by setting the date while in the adjustment 'danger zone' (9 PM - 3 AM).",
        lore_fact: "Screw-down crowns ensure 300 m water resistance, provided you remember to actually screw them back down before jumping into the pool.",
    };

    pub const HANDS: Part = Part {
        name: "Hands",
        category: "Dial Assembly",
        function: "Friction-fitted onto concentric pinions (hour wheel, cannon pinion, fourth wheel arbor) to indicate time.",
        failure_mode: "Bent tips, scratched paint during removal, rubbing against the crystal or indices.",
        lore_fact: "The seconds hand pinion can be as thin as 0.2 mm. Pressing it down without a loupe requires surgical confidence or reckless optimism.",
    };

    pub const ALL_PARTS: &[&'static Part] = &[
        &CASE_BACK,
        &WINDING_ROTOR,
        &MAINSPRING,
        &BALANCE_WHEEL_HAIRSPRING,
        &ESCAPEMENT_PALLET_FORK,
        &CROWN_STEM,
        &HANDS
    ];
}
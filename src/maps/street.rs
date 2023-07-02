use crate::helpers::{Direction, Event};

pub struct Street;

impl Street {
    pub const BACKGROUND_COLOR: &'static str = "black";
    pub const HERO_POSITION: [u16; 2] = [5, 11];
    pub const LOWER_IMAGE: &'static str = "images/maps/StreetLower.png";
    pub const UPPER_IMAGE: &'static str = "images/maps/StreetUpper.png";
    pub const NPCS: [(
        &'static str,
        &'static str,
        f64,
        f64,
        &'static [(Event, Direction, u16)],
    ); 1] = [(
        "Rachel",
        "images/characters/people/npc8.png",
        24.0 * 16.0,
        9.0 * 16.0,
        &[(Event::Stand, Direction::Down, 1)],
    )];
    pub const NPC_CUTSCENES: [(
        &'static str,
        &'static [(
            &'static [&'static str],
            &'static [&'static [[&'static str; 2]]],
        )],
    ); 0] = [];
    pub const ACTION_CUTSCENES: [(
        [u16; 2],
        &'static [(
            &'static [&'static str],
            &'static [&'static [[&'static str; 2]]],
        )],
    ); 7] = [
        (
            [4, 10],
            &[(
                &[],
                &[&[
                    ["type", "changeMap"],
                    ["map", "DiningRoom"],
                    ["heroPosition", "6 11"],
                    ["direction", "up"],
                    ["repeat", "1"],
                ]],
            )],
        ),
        (
            [4, 11],
            &[(
                &[],
                &[&[
                    ["type", "changeMap"],
                    ["map", "DiningRoom"],
                    ["heroPosition", "6 11"],
                    ["direction", "up"],
                    ["repeat", "1"],
                ]],
            )],
        ),
        (
            [4, 12],
            &[(
                &[],
                &[&[
                    ["type", "changeMap"],
                    ["map", "DiningRoom"],
                    ["heroPosition", "6 11"],
                    ["direction", "up"],
                    ["repeat", "1"],
                ]],
            )],
        ),
        (
            [4, 13],
            &[(
                &[],
                &[&[
                    ["type", "changeMap"],
                    ["map", "DiningRoom"],
                    ["heroPosition", "6 11"],
                    ["direction", "up"],
                    ["repeat", "1"],
                ]],
            )],
        ),
        (
            [29, 9],
            &[(
                &[],
                &[&[
                    ["type", "changeMap"],
                    ["map", "PizzaShop"],
                    ["heroPosition", "5 11"],
                    ["direction", "up"],
                    ["repeat", "1"],
                ]],
            )],
        ),
        (
            [5, 9],
            &[(
                &[],
                &[&[
                    ["type", "changeMap"],
                    ["map", "Kitchen"],
                    ["heroPosition", "5 9"],
                    ["direction", "up"],
                    ["repeat", "1"],
                ]],
            )],
        ),
        (
            [25, 5],
            &[(
                &[],
                &[&[
                    ["type", "changeMap"],
                    ["map", "StreetNorth"],
                    ["heroPosition", "7 15"],
                    ["direction", "up"],
                    ["repeat", "1"],
                ]],
            )],
        ),
    ];
    pub const PIZZA_STONES: [(
        [u16; 2],
        &'static str,
        &'static [&'static [[&'static str; 2]]],
    ); 0] = [];
    pub const ITEMS: [(
        [u16; 2],
        &'static str,
        &'static [&'static [[&'static str; 2]]],
    ); 0] = [];
    pub const WALLS: [[u16; 2]; 87] = [
        [3, 10],
        [3, 11],
        [3, 12],
        [3, 13],
        [4, 14],
        [5, 14],
        [6, 14],
        [7, 14],
        [8, 14],
        [9, 14],
        [10, 14],
        [11, 14],
        [12, 14],
        [13, 14],
        [14, 14],
        [15, 14],
        [16, 14],
        [17, 14],
        [18, 14],
        [19, 14],
        [20, 14],
        [21, 14],
        [22, 14],
        [23, 14],
        [24, 14],
        [25, 14],
        [26, 14],
        [27, 14],
        [28, 14],
        [29, 14],
        [30, 14],
        [31, 14],
        [32, 14],
        [33, 14],
        [34, 13],
        [34, 12],
        [34, 11],
        [34, 10],
        [4, 9],
        [5, 8],
        [6, 9],
        [7, 9],
        [8, 9],
        [9, 9],
        [10, 9],
        [11, 9],
        [12, 9],
        [13, 8],
        [14, 8],
        [15, 7],
        [16, 7],
        [17, 7],
        [18, 7],
        [19, 7],
        [20, 7],
        [21, 7],
        [22, 7],
        [23, 7],
        [24, 7],
        [24, 6],
        [26, 6],
        [26, 5],
        [24, 5],
        [26, 7],
        [27, 7],
        [28, 8],
        [28, 9],
        [29, 8],
        [30, 9],
        [31, 9],
        [32, 9],
        [33, 9],
        // objects
        [16, 9],
        [16, 10],
        [16, 11],
        [17, 9],
        [17, 10],
        [17, 11],
        [18, 11],
        [19, 11],
        [25, 11],
        [25, 10],
        [25, 9],
        [26, 11],
        [26, 10],
        [26, 9],
        // npcs
        [24, 9],
    ];
}

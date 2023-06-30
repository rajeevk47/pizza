use crate::helpers::{Direction, Event};

pub struct DiningRoom;

impl DiningRoom {
    pub const BACKGROUND_COLOR: &'static str = "black";
    pub const HERO_POSITION: [u16; 2] = [7, 3];
    pub const LOWER_IMAGE: &'static str = "images/maps/DiningRoomLower.png";
    pub const UPPER_IMAGE: &'static str = "images/maps/DiningRoomUpper.png";
    pub const NPCS: [(
        &'static str,
        &'static str,
        f64,
        f64,
        &'static [(Event, Direction, u16)],
    ); 0] = [];
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
    ); 2] = [
        (
            [7, 3],
            &[(
                &[],
                &[&[
                    ["type", "changeMap"],
                    ["map", "DemoRoom"],
                    ["heroPosition", "5 9"],
                    ["direction", "up"],
                    ["repeat", "1"],
                ]],
            )],
        ),
        (
            [6, 12],
            &[(
                &[],
                &[&[
                    ["type", "changeMap"],
                    ["map", "Street"],
                    ["heroPosition", "5 11"],
                    ["direction", "right"],
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
    pub const WALLS: [[u16; 2]; 55] = [
        [0, 4],
        [0, 6],
        [0, 7],
        [0, 8],
        [0, 9],
        [0, 10],
        [0, 11],
        [1, 12],
        [2, 12],
        [3, 12],
        [4, 12],
        [5, 12],
        [6, 13],
        [7, 12],
        [8, 12],
        [9, 12],
        [10, 12],
        [11, 12],
        [12, 12],
        [13, 11],
        [13, 6],
        [13, 8],
        [13, 9],
        [13, 10],
        [1, 3],
        [2, 3],
        [3, 3],
        [4, 3],
        [5, 3],
        [7, 2],
        [8, 3],
        [9, 4],
        // objects
        [1, 5],
        [2, 5],
        [3, 5],
        [4, 5],
        [6, 5],
        [6, 4],
        [10, 5],
        [11, 5],
        [12, 5],
        [2, 7],
        [3, 7],
        [4, 7],
        [7, 7],
        [8, 7],
        [9, 7],
        [2, 10],
        [3, 10],
        [4, 10],
        [7, 10],
        [8, 10],
        [9, 10],
        [11, 7],
        [12, 7],
    ];
}

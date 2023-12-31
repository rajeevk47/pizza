use serde::{Serialize, Deserialize};
use wasm_bindgen::UnwrapThrowExt;
use web_sys::HtmlImageElement;

pub struct Image {
    image: HtmlImageElement,
}

impl Image {
    pub fn new(src: &str) -> Self {
        let image = HtmlImageElement::new().unwrap();
        image.set_src(src);

        Self { image }
    }

    pub fn loaded(&self) -> bool {
        self.image.complete()
    }

    pub fn image_ref(&self) -> &HtmlImageElement {
        &self.image
    }
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum Event {
    Walk,
    Stand,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NpcCutscenes {
    pub name: String,
    pub scenes: Vec<Scene>,
}
 
#[derive(Debug, Serialize, Deserialize)]
pub struct ActionCutscenes {
    pub position: [u16; 2],
    pub scenes: Vec<Scene>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Action {
    pub event: Event,
    pub direction: Direction,
    pub repeat: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Scene {
    pub flags: Vec<String>,
    pub scene: Vec<Vec<[String; 2]>>,
}

pub struct DirectionInput {
    pub held_directions: Vec<Direction>,
}

impl DirectionInput {
    pub const MAP: [Direction; 4] = [
        Direction::Down,
        Direction::Left,
        Direction::Right,
        Direction::Up,
    ];

    pub fn new() -> Self {
        Self {
            held_directions: Vec::with_capacity(4),
        }
    }

    pub fn add(&mut self, direction_number: usize) {
        if !self.held_directions.contains(&Self::MAP[direction_number]) {
            self.held_directions.push(Self::MAP[direction_number].clone());
        }
    }

    pub fn remove(&mut self, direction_number: usize) {
        let index = self
            .held_directions
            .iter()
            .position(|e| *e == Self::MAP[direction_number])
            .unwrap();
        self.held_directions.remove(index);
    }
}


pub fn get_direction(direction: &str) -> Direction {
    match direction {
        "left" => Direction::Left,
        "right" => Direction::Right,
        "up" => Direction::Up,
        _ => Direction::Down,
    }
}

pub struct Movement {
    pub progress_remaining: u16,
    pub moveable: bool,
}

impl Movement {
    pub fn new(starting_progress: u16) -> Self {
        Self {
            progress_remaining: starting_progress,
            moveable: true,
        }
    }

    pub fn can_move(&mut self, walls: &mut Vec<[u16; 2]>, dx: &f64, dy: &f64, direction: &Direction) {
        let elem = get_next_place(direction, dx, dy);

        self.moveable = match walls.contains(&elem) {
            true => false,
            false => {
                walls.remove(
                    walls
                        .iter()
                        .position(|pair| *pair == [*dx as u16, *dy as u16])
                        .unwrap_throw(),
                );
                walls.push(elem);
                true
            }
        }
    }
}

pub fn get_next_place(direction: &Direction, dx: &f64, dy: &f64) -> [u16; 2] {
    match direction {
        Direction::Down => [*dx as u16, *dy as u16 + 16],
        Direction::Up => [*dx as u16, *dy as u16 - 16],
        Direction::Right => [*dx as u16 + 16, *dy as u16],
        Direction::Left => [*dx as u16 - 16, *dy as u16],
    }
}

pub fn get_opposite_direction(direction: &Direction) -> Direction {
    match direction {
        Direction::Down => Direction::Up,
        Direction::Up => Direction::Down,
        Direction::Right => Direction::Left,
        Direction::Left => Direction::Right,
    }
}

pub struct Animation {
    pub count: usize,
    pub sx: f64,
    pub sy: f64,
}

impl Animation {
    pub const FRAMES_PER_STEP: u16 = 8;

    pub const STEPS: [[[u16; 2]; 4]; 4] = [
        [[0, 0], [32, 0], [0, 0], [96, 0]],
        [[0, 32], [32, 32], [0, 32], [96, 32]],
        [[0, 64], [32, 64], [0, 64], [96, 64]],
        [[0, 96], [32, 96], [0, 96], [96, 96]],
    ];

    pub const STONE_STEPS: [[u16; 2]; 2] = [[32, 0], [0, 0]];

    pub fn new(direction: Option<&Direction>) -> Self {
        let s = match direction {
            Some(x) => match x {   
                Direction::Down => Self::STEPS[0][0],
                Direction::Right => Self::STEPS[1][0],
                Direction::Up => Self::STEPS[2][0],
                Direction::Left => Self::STEPS[3][0],
            }
            None => [0, 0],
        };

        Self {
            count: 0,
            sx: s[0] as f64,
            sy: s[1] as f64,
        }
    }

    pub fn selected_frame(&mut self, direction: &Direction, count: usize) {
        let index = match direction {
            Direction::Down => 0,
            Direction::Right => 1,
            Direction::Up => 2,
            Direction::Left => 3,
        };

        self.sx = Self::STEPS[index][count][0] as f64;
        self.sy = Self::STEPS[index][count][1] as f64;
        self.count = count;
    }

    pub fn toggle(&mut self, direction: &Direction) {
        let index = match direction {
            Direction::Down => 0,
            Direction::Right => 1,
            Direction::Up => 2,
            Direction::Left => 3,
        };

        match &mut self.count {
            x if *x < Self::STEPS.len() - 1 => *x += 1,
            x => *x = 0,
        }

        self.sx = Self::STEPS[index][self.count][0] as f64;
        self.sy = Self::STEPS[index][self.count][1] as f64;
    }
    
    pub fn toggle_stone(&mut self) {
        self.sx = Self::STONE_STEPS[1][0] as f64;
        self.sy = Self::STONE_STEPS[1][1] as f64;
    }
}

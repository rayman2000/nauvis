#![allow(dead_code, unused_variables)]

use serde::{Deserialize, Serialize};
use serde_repr::Deserialize_repr;

#[derive(Debug)]
pub struct Blueprint {
    pub entities: Vec<Entity>,
}

impl Blueprint {
    pub fn new(entities: Vec<Entity>) -> Self {
        Blueprint { entities }
    }

    pub fn entityAt(&self, pos: &Position) -> Option<&Entity> {
        self.entities
            .iter()
            .find(|entity| entity.get_positions().contains(pos))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, PartialOrd)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl std::ops::Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Position {
    fn scale(self, f: f64) -> Position {
        Position {
            x: self.x * f,
            y: self.y * f,
        }
    }

    fn shift_one(self, d: Direction) -> Position {
        self + d.as_position()
    }

    fn shift_half(self, d: Direction) -> Position {
        self + d.as_position().scale(0.5)
    }
}

// All float values are precisely representable as integers.
impl std::cmp::Eq for Position {}
impl std::cmp::Ord for Position {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize_repr, Clone, Copy)]
#[repr(i32)]
#[serde(untagged)]
pub enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

impl Direction {
    fn as_position(&self) -> Position {
        // TODO: check whether the offsets are correct
        let (x, y) = match self {
            Direction::North => (0.0, 1.0),
            Direction::East => (1.0, 0.0),
            Direction::South => (0.0, -1.0),
            Direction::West => (-1.0, 0.0),
        };

        Position { x, y }
    }

    const fn cw(self) -> Direction {
        use Direction::*;
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }

    const fn ccw(self) -> Direction {
        use Direction::*;
        match self {
            North => West,
            East => North,
            South => East,
            West => South,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Entity {
    pub position: Position,
    pub direction: Direction,
    pub entity_number: i32,
    #[serde(flatten)]
    pub ty: EntityType,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "name", rename_all = "kebab-case")]
pub enum EntityType {
    TransportBelt,
    #[serde(rename = "assembling-machine-1")]
    AssemblingMachine {
        recipe: String,
    },
    FilterInserter {
        filters: Vec<Filter>,
    },
    ElectricFurnace {},
    UndergroundBelt {
        #[serde(rename = "type")]
        belt_type: String,
    },
    ChemicalPlant {
        recipe: String,
    },
    Splitter {
        filter: String,
        input_priority: String,
        output_priority: String,
    },
    StoneWall {},
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Filter {
    index: i32,
    name: String,
}

impl Entity {
    pub fn get_positions(&self) -> Vec<Position> {
        match &self.ty {
            EntityType::TransportBelt => vec![self.position],
            EntityType::AssemblingMachine { recipe: _ } => {
                get_surrounding(self.position)
            }
            EntityType::FilterInserter { filters: _ } => vec![self.position],
            EntityType::ElectricFurnace {} => get_surrounding(self.position),
            EntityType::UndergroundBelt { belt_type: _ } => vec![self.position],
            EntityType::ChemicalPlant { recipe: _ } => {
                get_surrounding(self.position)
            }
            EntityType::Splitter {
                filter: _,
                input_priority: _,
                output_priority: _,
            } => todo!(),
            EntityType::StoneWall {} => vec![self.position],
        }
    }

    pub fn output_positions(&self) -> Vec<Position> {
        match &self.ty {
            EntityType::FilterInserter { filters: _ } => todo!(),
            _ => self.get_positions(),
        }
    }

    pub fn input_positions(&self) -> Vec<Position> {
        match &self.ty {
            EntityType::FilterInserter { filters: _ } => todo!(),
            _ => self.get_positions(),
        }
    }
}

impl std::ops::Deref for Entity {
    type Target = EntityType;

    fn deref(&self) -> &Self::Target {
        &self.ty
    }
}

impl EntityType {
    pub fn is_beltlike(&self) -> bool {
        match self {
            EntityType::TransportBelt => true,
            EntityType::UndergroundBelt { .. } => true,
            EntityType::Splitter { .. } => true,
            _ => false,
        }
    }
}

fn get_surrounding(position: Position) -> Vec<Position> {
    vec![
        Position {
            x: position.x - 1.0,
            y: position.y - 1.0,
        },
        Position {
            x: position.x - 1.0,
            y: position.y,
        },
        Position {
            x: position.x - 1.0,
            y: position.y + 1.0,
        },
        Position {
            x: position.x,
            y: position.y - 1.0,
        },
        Position {
            x: position.x,
            y: position.y,
        },
        Position {
            x: position.x,
            y: position.y + 1.0,
        },
        Position {
            x: position.x + 1.0,
            y: position.y - 1.0,
        },
        Position {
            x: position.x + 1.0,
            y: position.y,
        },
        Position {
            x: position.x + 1.0,
            y: position.y + 1.0,
        },
    ]
}

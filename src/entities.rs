#![allow(dead_code, unused_variables)]

use std::fmt;

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

    pub fn entity_at(&self, pos: &Position) -> Option<&Entity> {
        self.entities
            .iter()
            .find(|entity| entity.get_positions().contains(pos))
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, PartialOrd)]
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

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Position {
    pub fn new(x: f64, y: f64) -> Self {
        Position { x, y }
    }

    pub fn neighbours(&self) -> Vec<Position> {
        vec![
            self.shift_one(Direction::North),
            self.shift_one(Direction::East),
            self.shift_one(Direction::South),
            self.shift_one(Direction::West),
        ]
    }

    pub fn three_by_three(&self) -> Vec<Position> {
        vec![
            Position {
                x: self.x - 1.0,
                y: self.y - 1.0,
            },
            Position {
                x: self.x - 1.0,
                y: self.y,
            },
            Position {
                x: self.x - 1.0,
                y: self.y + 1.0,
            },
            Position {
                x: self.x,
                y: self.y - 1.0,
            },
            Position {
                x: self.x,
                y: self.y,
            },
            Position {
                x: self.x,
                y: self.y + 1.0,
            },
            Position {
                x: self.x + 1.0,
                y: self.y - 1.0,
            },
            Position {
                x: self.x + 1.0,
                y: self.y,
            },
            Position {
                x: self.x + 1.0,
                y: self.y + 1.0,
            },
        ]
    }

    fn scale(self, f: f64) -> Position {
        Position {
            x: self.x * f,
            y: self.y * f,
        }
    }

    pub fn shift_one(self, d: Direction) -> Position {
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

#[derive(Debug, Serialize, Deserialize_repr, Clone, Copy, PartialEq)]
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Entity {
    pub position: Position,
    pub direction: Direction,
    pub entity_number: i32,
    #[serde(flatten)]
    pub ty: EntityType,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(tag = "name", rename_all = "kebab-case")]
pub enum EntityType {
    TransportBelt,
    #[serde(rename = "assembling-machine-1")]
    AssemblingMachine {
        recipe: String,
    },
    FilterInserter {
        filters: Option<Vec<Filter>>,
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Filter {
    index: i32,
    name: String,
}

impl Entity {
    pub fn get_positions(&self) -> Vec<Position> {
        match &self.ty {
            EntityType::TransportBelt => vec![self.position],
            EntityType::AssemblingMachine { recipe: _ } => {
                self.position.three_by_three()
            }
            EntityType::FilterInserter { filters: _ } => vec![self.position],
            EntityType::ElectricFurnace {} => self.position.three_by_three(),
            EntityType::UndergroundBelt { belt_type: _ } => vec![self.position],
            EntityType::ChemicalPlant { recipe: _ } => {
                self.position.three_by_three()
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

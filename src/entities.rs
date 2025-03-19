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

    pub fn get_borders(&self) -> (i32, i32, i32, i32) {
        let (min_x, min_y, max_x, max_y) = self.entities.iter().flat_map(|entity| entity.get_positions()).fold((f64::MAX, f64::MAX, f64::MIN, f64::MIN), |(min_x, min_y, max_x, max_y), pos| {
            (min_x.min(pos.x), min_y.min(pos.y), max_x.max(pos.x), max_y.max(pos.y))
        });
        (min_x.floor() as i32, min_y.floor() as i32, max_x.ceil() as i32, max_y.ceil() as i32)

    }

    pub fn render(&self) -> String {
        let (min_x, min_y, max_x, max_y) = self.get_borders();

        println!("min_x: {}, min_y: {}, max_x: {}, max_y: {}", min_x, min_y, max_x, max_y);

        let mut grid: Vec<Vec<String>> = vec![vec!["  ".to_string(); (max_x - min_x) as usize]; (max_y - min_y) as usize];
        
        
        let x_offset = min_x as f64 + 0.5;
        let y_offset = min_y as f64 + 0.5;

        for ent in &self.entities {
            for pos in ent.get_positions() {
                
                let x = (pos.x - x_offset) as usize;
                let y = (pos.y - y_offset) as usize;

                println!("Inserting {:?} at ({},{})", ent, x, y);
                grid.get_mut(y ).unwrap().insert(x, ent.render());
            }
        }
    grid.iter()
    .map(|row| row.join(""))
    .collect::<Vec<String>>()
    .join("\n")
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

    // For more arrows, see http://xahlee.info/comp/unicode_arrows.html
    pub fn render(&self) -> String {
        match &self.ty {
            EntityType::FilterInserter { filters: _ } => match self.direction {
                Direction::North => "⏫".to_string(),
                Direction::East => "⏩".to_string(),
                Direction::South => "⏬".to_string(),
                Direction::West => "⏪".to_string(),
            }
            EntityType::AssemblingMachine { recipe: _} => "🛠️".to_string(),
            EntityType::TransportBelt => match self.direction {
                Direction::North => "⬆️".to_string(),
                Direction::East => "➡️".to_string(),
                Direction::South => "⬇️".to_string(),
                Direction::West => "⬅️".to_string(),
            },
            EntityType::ElectricFurnace {  } => "🔥".to_string(),
            // TODO how does one differentiate in and out? ↤ ↦ ↥ ↧
            EntityType::UndergroundBelt { belt_type } => match self.direction {
                Direction::North => "⤒".to_string(),
                Direction::East => "⇥".to_string(),
                Direction::South => "⤓".to_string(),
                Direction::West => "⇤".to_string(),
            },
            EntityType::ChemicalPlant { recipe } => todo!(),
            EntityType::Splitter { filter, input_priority, output_priority } => match self.direction {
                Direction::North => "⇞".to_string(),
                Direction::East => "⇻".to_string(),
                Direction::South => "⇟".to_string(),
                Direction::West => "⇺".to_string(),
            },
            EntityType::StoneWall {  } => "🧱".to_string(),
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

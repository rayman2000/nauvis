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
        self.entities.iter().find(|entity| entity.get_positions().contains(pos))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Serialize, Deserialize_repr)]
#[repr(i32)]
#[serde(untagged)]
pub enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Entity {
    position: Position,
    direction: Direction,
    entity_number: i32,
    #[serde(flatten)]
    ty: EntityType,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "name", rename_all = "kebab-case")]
enum EntityType {
    TransportBelt,
    #[serde(rename = "assembling-machine-1")]
    AssemblingMachine  {
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
            EntityType::AssemblingMachine { recipe: _ } => get_surrounding(self.position),
            EntityType::FilterInserter { filters: _ } => vec![self.position],
            EntityType::ElectricFurnace {  } => get_surrounding(self.position),
            EntityType::UndergroundBelt { belt_type: _ } => vec![self.position],
            EntityType::ChemicalPlant { recipe: _ } => get_surrounding(self.position),
            EntityType::Splitter { filter: _, input_priority: _, output_priority: _ } => todo!(),
            EntityType::StoneWall {  } => vec![self.position],
        }
    }

    pub fn output_positions(&self)-> Vec<Position> {
        match &self.ty {
            EntityType::FilterInserter { filters: _ } => todo!(),
            _ => self.get_positions()
        }
    }

    pub fn input_positions(&self)-> Vec<Position> {
        match &self.ty {
            EntityType::FilterInserter { filters: _ } => todo!(),
            _ => self.get_positions()
        }
    }
}

fn get_surrounding(position: Position) -> Vec<Position> {
    vec![
        Position {x: position.x - 1.0, y: position.y - 1.0},
        Position { x: position.x - 1.0, y: position.y  },
        Position { x: position.x - 1.0, y: position.y + 1.0 },
        Position {x: position.x, y: position.y - 1.0},
        Position { x: position.x, y: position.y  },
        Position { x: position.x, y: position.y + 1.0 },
        Position {x: position.x + 1.0, y: position.y - 1.0},
        Position { x: position.x + 1.0, y: position.y  },
        Position { x: position.x + 1.0, y: position.y + 1.0 },
            
    ]
}
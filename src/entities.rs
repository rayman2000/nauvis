use serde::{Deserialize, Serialize};
use serde_repr::Deserialize_repr;

#[derive(Debug)]
pub struct Blueprint {
    entities: Vec<Entity>,
}

impl Blueprint {
    pub fn new(entities: Vec<Entity>) -> Self {
        Blueprint { entities }
    }

    pub fn entityAt(&self, pos: Position) -> Option<Entity> {
        self.entities.iter().find(|entity| entity.get_positions().contains(pos))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
struct Position {
    x: f64,
    y: f64,
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

    fn get_positions(&self) -> Vec<Position> {
        match &self.ty {
            EntityType::TransportBelt => vec![self.position],
            EntityType::AssemblingMachine { recipe } => get_surrounding(self.position),
            EntityType::FilterInserter { filters } => vec![self.position],
            EntityType::ElectricFurnace {  } => get_surrounding(self.position),
            EntityType::UndergroundBelt { belt_type } => vec![self.position],
            EntityType::ChemicalPlant { recipe } => get_surrounding(self.position),
            EntityType::Splitter { filter, input_priority, output_priority } => todo!(),
            EntityType::StoneWall {  } => vec![self.position],
        }
    }

    fn output_positions(&self)-> Vec<Position> {
        match &self.ty {
            EntityType::FilterInserter { filters } => todo!(),
            _ => self.get_positions()
        }
    }

    fn input_positions(&self)-> Vec<Position> {
        match &self.ty {
            EntityType::FilterInserter { filters } => todo!(),
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